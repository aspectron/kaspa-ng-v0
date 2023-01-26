#![allow(dead_code)]

use crate::prelude::*;
use workflow_ux::controls::prelude::*;
use workflow_ux::result::Result;

#[derive(Clone)]
pub struct Hidden {
    value: Arc<Mutex<Option<String>>>,
}

unsafe impl Send for Hidden {}

impl Hidden {
    pub fn new(_pane: &ElementLayout, _attributes: &Attributes, _docs: &Docs) -> Result<Self> {
        Ok(Self {
            value: Arc::new(Mutex::new(None)),
        })
    }

    pub fn value(&self) -> Result<Option<String>> {
        let value = self.value.lock()?.clone();
        Ok(value)
    }

    pub fn set_value(&self, value: Option<String>) -> Result<()> {
        *self.value.lock()? = value;
        Ok(())
    }
}


#[form(title="Import Wallet")]
struct ImportForm{
    #[field()]
    mnemonic: Mnemonic,

    #[field(skip=true)]
    stage: Hidden
}


#[async_trait_without_send]
impl FormHandler for ImportForm {
    async fn load(&self) -> Result<()> {
        self.set_submit_btn_text(i18n("Next"))?;
        Ok(())
    }
    async fn submit(&self) -> Result<()> {
        let value = self.mnemonic.value();
        log_trace!("mnemonics: {value}");
        Ok(())
    }
}
