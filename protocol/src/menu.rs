
use crate::prelude::*;
use workflow_ux::app_menu::AppMenu as Inner;
use workflow_ux::{app_drawer::AppDrawer, main_menu::MainMenu, result::Result};

#[derive(Debug, Clone)]
pub struct AppMenu {
    inner: Arc<Inner>,
    drawer: Arc<AppDrawer>
}

impl AppMenu {

    pub fn new() -> Result<Self> {

        let inner = Arc::new(Self::create_inner()?);
        let drawer = AppDrawer::get("flow-app-drawers")?;
        let menu = Self{
            inner,
            drawer: Arc::new(drawer)
        };
        menu.init()?;
        Ok(menu)
    }

    fn init(&self)->Result<()>{
        let this = self.clone();
        let mut bottom = self.inner.bottom.lock().expect("Unable to lock BottomMenu");
        bottom.add_default_item_with_callback("Menu", Icon::Wallet, move |_ev| ->Result<()>{
            this.drawer.toggle_left_drawer();
            Ok(())
        })?;
        bottom.add_default_item("TBD", Icon::Work)?;
        bottom.add_default_item("Help", Icon::Help)?;
        let this = self.clone();
        bottom.add_default_item_with_callback("Wallet", Icon::Wallet, move |_ev| ->Result<()>{
            this.drawer.toggle_right_drawer();
            Ok(())
        })?;

        bottom.show()?;
        
        Ok(())
    }

    fn create_inner()->Result<Inner>{
        let menu = Inner::new(
            "#app-menu",
            Some("#sub-menu"),
            "#kaspa-bottom-nav",
            Some("body")
        )?;
        Ok(menu)
    }

    pub fn inner(&self)->Arc<Inner>{
        self.inner.clone()
    }

    pub fn main(&self)->Arc<MainMenu>{
        self.inner.main.clone()
    }

    pub fn popup(&self)->Arc<PopupMenu>{
        let menu = self.inner.popup.as_ref().expect("PopupMenu is not initialized.");
        menu.clone()
    }

}
