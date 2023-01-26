use crate::prelude::*;
use crate::result::Result;
use std::sync::Arc;
use url::Url;
use workflow_core::channel::{unbounded, Receiver, Sender};

#[derive(Debug, Clone)]
pub enum Event {
    Halt,
    Balance(Id), //ID just for random number test
    Blocks(u32),
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Application {
    workspace: Arc<Workspace>,
    #[allow(dead_code)]
    inner: Arc<workflow_ux::application::Application>,
    #[wasm_bindgen(skip)]
    pub user: User,
    channels: Arc<Mutex<HashMap<Id, Sender<Event>>>>,
}

static mut APPLICATION: Option<Application> = None;

#[wasm_bindgen]
impl Application {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Application> {
        let inner = Arc::new(workflow_ux::application::Application::new(Some(
            "kaspa-app",
        ))?);
        //log_trace!("AAAAA Workspace::new():started");
        let workspace = Arc::new(Workspace::new()?);
        //log_trace!("AAAAA Workspace::new():finished");
        let user = User::new();

        let app = Application {
            workspace,
            inner,
            user,
            channels: Arc::new(Mutex::new(HashMap::default())),
        };

        unsafe {
            APPLICATION = Some(app.clone());
        }
        Ok(app)
    }

    pub async fn init(&self) -> Result<()> {
        let url = self.location();
        let fragment = url.fragment().unwrap_or("");
        let mut module_load_order = [
            "header",
            "status",
            "wallet",
            "console",
            "network",
            "resources",
        ]
        .to_vec();

        let mut module_disable_list: Vec<&str> = [
            // "test"
        ]
        .to_vec();

        if !fragment.contains("test") {
            module_disable_list.push("test");
        } else {
            let mut new_order = Vec::from(["test"]);
            new_order.append(&mut module_load_order);
            module_load_order = new_order;
        }
        self.inner
            .load_modules(
                workflow_ux::wasm::workflow()?,
                &module_load_order,
                &module_disable_list,
            )
            .await
            .expect("Application::load_modules() failed");
        match self.user.restore().await {
            Ok(authority) => {
                log_trace!("Application::user restoring authority {:?}", authority);
            }
            Err(e) => {
                log_warning!("Application::user unable to restore user authority - {}", e);
            }
        }

        Ok(())
    }
}

impl Application {
    pub fn workspace(&self) -> Arc<Workspace> {
        self.workspace.clone()
    }
    pub fn location(&self) -> Url {
        self.inner.location()
    }
    pub fn reload(&self) {
        log_trace!("application reload...");
        window()
            .location()
            .reload()
            .expect("Application::reload() failure");
    }

    pub fn register_event_channel() -> (Id, Sender<Event>, Receiver<Event>) {
        application().register_event_channel_impl()
    }

    pub fn register_event_channel_impl(&self) -> (Id, Sender<Event>, Receiver<Event>) {
        let (sender, receiver) = unbounded();
        let id = Id::new();
        self.channels.lock().unwrap().insert(id, sender.clone());
        (id, sender, receiver)
    }

    pub fn unregister_event_channel(id: Id) {
        application().unregister_event_channel_impl(id);
    }

    pub fn unregister_event_channel_impl(&self, id: Id) {
        self.channels.lock().unwrap().remove(&id);
    }

    pub fn reflect(&self, event: Event) {
        let channels = self.channels.lock().unwrap();
        for (_, sender) in channels.iter() {
            match sender.try_send(event.clone()) {
                Ok(_) => {}
                Err(err) => {
                    log_error!(
                        "Aplication reflect: error reflecting event {:?}: {:?}",
                        event,
                        err
                    );
                }
            }
        }
    }
}

pub fn global() -> Result<Application> {
    let clone = unsafe {
        (APPLICATION)
            .as_ref()
            .ok_or_else(|| panic!("Application global is not registered"))
            .unwrap()
            .clone()
    };
    Ok(clone)
}

pub fn application() -> Application {
    global().expect("Missing global application object")
}
