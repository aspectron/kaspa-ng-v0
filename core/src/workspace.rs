
use crate::menu::AppMenu;
use workflow_ux::prelude::*;
use workflow_ux::result::Result;


pub fn workspace() -> std::sync::Arc<Workspace> {
    crate::application::global().expect("Missing global application object").workspace()
}

pub struct Workspace {
    inner : workspace::Workspace,
    menu: Arc<AppMenu>
}

impl Workspace {
    pub fn new() -> Result<Workspace> {
        let menu = AppMenu::new()?;
        let inner = workspace::Workspace::new(
            "#app-header",
            "#app-status",
            "#app-main",
            "#app-sidebar",
            menu.inner()
        )?;

        let workspace = Workspace {
            inner,
            menu: Arc::new(menu)
        };

        Ok(workspace)
    }

    pub fn header(&self) -> Arc<Container> {
        self.inner.header()
    }

    pub fn menu(&self) -> Arc<AppMenu> {
        self.menu.clone()
    }

    pub fn status(&self) -> Arc<Container> {
        self.inner.status()
    }

    pub fn main(&self) -> Arc<Container> {
        self.inner.main()
    }

    pub fn sidebar(&self) -> Arc<ContainerStack> {
        self.inner.sidebar()
    }

}
