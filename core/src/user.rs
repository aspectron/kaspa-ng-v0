//use std::sync::atomic::{AtomicBool, Ordering};
use kaspa_core::result::Result;
use kaspa_core::prelude::*;
//use crate::profile::wallet::{WalletConnectHandler, wallet_adaptor};
//use workflow_ux::local_storage;
//use serde::{Serialize, Deserialize};
//use crate::wallet::module::Wallet as WalletModule;
//use workflow_ux::dialog::Dialog;


#[derive(Clone)]
pub struct User {
    //pending : Arc<AtomicBool>,
}

impl User {
    pub fn new() -> Self {
        User {
            //pending: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn get() -> Result<User> {
        Ok(application().user)
    }

    pub fn store(&self) -> Result<()> {
        
        Ok(())
    }

    pub async fn restore(&self) -> Result<()> {
        
        Ok(())
    }

    pub fn reset(&self) -> Result<()> {
        
        Ok(())
    }

    pub async fn is_present(&self) -> Result<bool> {
        Ok(true)
    }

    pub async fn block(&self, _module : Arc<dyn ModuleInterface>) -> Result<bool> {
 
        Ok(true)

    }

}

/*
impl WalletConnectHandler for User{
    fn on_wallet_connect(&self)->workflow_ux::result::Result<()> {
        //workspace().main().element().set_attribute("wallet-connected", "true")?;
        document().body().unwrap().set_attribute("wallet-connected", "true")?;
        Ok(())
    }
}
*/
