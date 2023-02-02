#![allow(dead_code)]

use crate::prelude::*;
use workflow_ux::controls::prelude::*;
use workflow_ux::result::Result;

#[derive(Clone)]
pub struct FormStageIndex {
    value: Arc<Mutex<u8>>,
}

unsafe impl Send for FormStageIndex {}

impl FormStageIndex {
    pub fn new(_pane: &ElementLayout, _attributes: &Attributes, _docs: &Docs) -> Result<Self> {
        Ok(Self {
            value: Arc::new(Mutex::new(1)),
        })
    }

    pub fn value(&self) -> Result<u8> {
        let value = *self.value.lock()?;
        Ok(value)
    }

    pub fn set_value(&self, value: u8) -> Result<()> {
        *self.value.lock()? = value;
        Ok(())
    }
}

#[group(title = "Please enter seeds")]
struct ImportFormStage1 {
    #[field()]
    mnemonic: Mnemonic,
}

#[group(title = "Confirming seeds")]
struct ImportFormStage2 {
    #[field(label = "1st word")]
    input1: Input,

    #[field(label = "4th word")]
    input2: Input,
}

#[group(title = "Create wallet password")]
struct ImportFormStage3 {
    #[field(type = "password", label = "Password")]
    password: Input,

    #[field(type = "password", label = "Confirm Password")]
    password_confirm: Input,
}

struct FormStages {
    pub stages: Vec<Arc<dyn FormHandler>>,
    pub index: u8,
}

#[form(title = "Import Wallet")]
struct ImportForm {
    stage1: ImportFormStage1,
    stage2: ImportFormStage2,
    stage3: ImportFormStage3,

    #[field(skip = true)]
    stage_index: FormStageIndex,
}

#[async_trait_without_send]
impl FormHandler for ImportForm {
    async fn load(&self) -> Result<()> {
        //self.set_stage_index(self.stage_index.value()?)?;
        Ok(())
    }

    async fn submit(&self) -> Result<()> {
        match self.stage_index.value()? {
            1 => {
                let value = self.stage1.mnemonic.value();
                log_trace!("mnemonics: {value}");
                //self.set_stage_index(2)?;
            }
            2 => {
                let word1 = self.stage2.input1.value();
                let word2 = self.stage2.input2.value();
                log_trace!("word1: {word1}");
                log_trace!("word2: {word2}");
                //self.set_stage_index(3)?;
            }
            _ => {}
        }

        Ok(())
    }
}
