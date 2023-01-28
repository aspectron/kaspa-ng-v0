use crate::application::Event;
use crate::network::module::Network;
use crate::prelude::*;
use js_sys::{Array, Date, Object};
use workflow_core::channel::Sender;
use workflow_ux::controls::prelude::*;
use workflow_ux::events::*;
use workflow_ux::result::Result;
use workflow_wasm::options::OptionsTrait;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type BadgeData;
}

impl workflow_wasm::options::OptionsTrait for BadgeData {}

impl From<(Date, u32)> for BadgeData {
    fn from(value: (Date, u32)) -> Self {
        Self::new()
            .set("date", value.0.into())
            .set("value", value.1.into())
    }
}

#[derive(HtmlView)]
pub struct NodeView {
    event_sender: Arc<Mutex<Option<Sender<Event>>>>,
    html: Arc<Mutex<Option<Arc<view::Html>>>>,
    badges: Arc<Mutex<HashMap<String, Badge>>>,
    blocks: Arc<Mutex<Array>>,
}

#[workflow_async_trait]
impl Evict for NodeView {
    async fn evict(self: Arc<Self>) -> Result<bool> {
        log_info!("NodeView: view_evict");
        //self.unsubscribe()?;
        Ok(false)
    }
}

impl NodeView {
    pub fn new(module: Arc<Network>) -> Result<Arc<Self>> {
        let view = Arc::new(Self {
            event_sender: Arc::new(Mutex::new(None)),
            html: Arc::new(Mutex::new(None)),
            badges: Arc::new(Mutex::new(HashMap::new())),
            blocks: Arc::new(Mutex::new(Array::new())),
        });

        view.clone().init(module)?;

        Ok(view)
    }

    fn init(self: Arc<Self>, module: Arc<Network>) -> Result<()> {
        let options = BadgeOptions::default()
            //.suffix(" / SEC")
            .align("right")
            .colon(true);
        //.sampler("test-key");

        let badge = Badge::create("Blocks", options)?;

        self.badges
            .lock()?
            .insert("blocks".to_string(), badge.clone());

        let view = view::Html::try_new(
            Some(module),
            html! {
                <div class="node-view">
                    <span class="value" @balance></span>
                    <div>{badge}</div>
                </div>
            }?,
        )?;

        *self.html.lock()? = Some(view);
        Ok(())
    }

    pub fn subscribe(self: Arc<Self>) -> Result<()> {
        if self.event_sender.lock()?.as_ref().is_some() {
            return Ok(());
        }

        let (id, sender, receiver) = Application::register_event_channel();
        *self.event_sender.lock()? = Some(sender);

        let this = Arc::downgrade(&self);
        let this2 = Arc::downgrade(&self);
        subscribe(
            receiver,
            move |event| -> CallbackResult {
                let view = this.clone();
                Box::pin(async move {
                    if let Some(c) = view.upgrade() {
                        c.digest_event(event).await
                    } else {
                        Ok(false)
                    }
                })
            },
            move || {
                Application::unregister_event_channel(id);
                if let Some(c) = this2.clone().upgrade() {
                    *c.event_sender.lock().unwrap() = None;
                }
            },
        )?;

        Ok(())
    }

    pub fn unsubscribe(&self) -> Result<()> {
        log_info!("NodeView unsubscribe");

        if let Some(sender) = self.event_sender.lock()?.as_ref() {
            sender.try_send(Event::Halt)?;
        }

        *self.event_sender.lock()? = None;

        Ok(())
    }

    async fn digest_event(self: Arc<Self>, event: Event) -> Result<bool> {
        log_info!("Node: got event: {:?}", event);
        match event {
            Event::Balance(balance) => {
                let binding = self.html.lock()?;
                let html_ = binding.as_ref().unwrap().html();
                let hooks = html_.hooks();
                hooks
                    .get("balance")
                    .unwrap()
                    .set_inner_html(&format!("{balance} KAS"));
            }

            Event::Blocks(blocks) => {
                let array = self.blocks.lock()?;
                array.push(&BadgeData::from((Date::new_0(), blocks)));
                while array.length() > 300 {
                    array.pop();
                }

                log_info!("array.len(): {}", array.length());
                let data = array.clone();
                if let Some(badge) = self.badges.lock()?.get("blocks") {
                    badge.redraw(&data, Some(&blocks.to_string()))?;
                }
            }

            Event::Halt => return Ok(false),
        }
        Ok(true)
    }
}
