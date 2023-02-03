#![allow(dead_code)]

use crate::prelude::*;
use workflow_ux::controls::prelude::*;
use workflow_ux::form::{FormStage, FormStages};
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

#[async_trait_without_send]
impl FormStage for ImportFormStage1 {
    async fn serialize(&self) -> Result<FormData> {
        let mut data = FormData::new(None);
        data.add_string("mnemonic", self.mnemonic.value());
        Ok(data)
    }
    async fn activate(&self) -> Result<()> {
        self.show(true)?;
        Ok(())
    }
    async fn deactivate(&self) -> Result<()> {
        self.show(false)?;
        Ok(())
    }
}

#[group(title = "Confirming seeds")]
struct ImportFormStage2 {
    #[field(label = "1st word")]
    input1: Input,

    #[field(label = "4th word")]
    input2: Input,
}

#[async_trait_without_send]
impl FormStage for ImportFormStage2 {
    async fn serialize(&self) -> Result<FormData> {
        let mut data = FormData::new(None);
        data.add_string("value1", self.input1.value());
        data.add_string("value2", self.input2.value());
        Ok(data)
    }
    async fn activate(&self) -> Result<()> {
        self.show(true)?;
        Ok(())
    }
    async fn deactivate(&self) -> Result<()> {
        self.show(false)?;
        Ok(())
    }
}

#[group(title = "Create wallet password")]
struct ImportFormStage3 {
    #[field(type = "password", label = "Password")]
    password: Input,

    #[field(type = "password", label = "Confirm Password")]
    password_confirm: Input,
}

#[async_trait_without_send]
impl FormStage for ImportFormStage3 {
    async fn serialize(&self) -> Result<FormData> {
        let mut data = FormData::new(None);
        data.add_string("password", self.password.value());
        data.add_string("password_confirm", self.password_confirm.value());
        Ok(data)
    }
    async fn activate(&self) -> Result<()> {
        self.show(true)?;
        Ok(())
    }
    async fn deactivate(&self) -> Result<()> {
        self.show(false)?;
        Ok(())
    }
}

#[form(title = "Import Wallet")]
struct ImportForm {
    #[field(title = "Step [INDEX]/2")]
    stages: FormStages,
}

#[async_trait_without_send]
impl FormHandler for ImportForm {
    async fn load(&self) -> Result<()> {
        self.stages
            .add_stage(Arc::new(ImportFormStage1::try_new()?))?;
        self.stages
            .add_stage(Arc::new(ImportFormStage2::try_new()?))?;
        self.stages
            .add_stage(Arc::new(ImportFormStage3::try_new()?))?;
        self.stages.activate_stage(0, Some(self.footer())).await?;
        Ok(())
    }

    async fn submit(&self) -> Result<()> {
        let data = self.stages.serialize_stage().await?;
        log_trace!("stage data: {:?}", data);

        /*
        // Or read data manually
        let index = self.stages.index()?;
        if index == 0{
            let stage = self.stages.stage_downcast_arc::<ImportFormStage1>()?;
            log_trace!("stage1.mnemonic: {:?}", stage.mnemonic.value());
        }
        */

        if !self.stages.next(Some(self.footer())).await? {
            log_trace!("complete data: {:#?}", self.stages.data()?);
            // submit data
        }

        Ok(())
    }
}
