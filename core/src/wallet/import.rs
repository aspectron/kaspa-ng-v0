#![allow(dead_code)]

use crate::prelude::*;
use rand::{thread_rng, RngCore};
use workflow_ux::controls::prelude::*;
use workflow_ux::dialog::show_error;
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
struct ImportFormStageConfirmingSeeds {
    #[field(label = "1st word")]
    input1: Input,

    #[field(label = "2nd word")]
    input2: Input,

    #[field(label = "3rd word")]
    input3: Input,

    #[field(label = "4th word")]
    input4: Input,

    #[field(label = "5th word")]
    input5: Input,

    #[field(skip = true)]
    #[rustfmt::skip]
    word_indexes: DataField::<Vec<u8>>,
}

#[async_trait_without_send]
impl FormStage for ImportFormStageConfirmingSeeds {
    async fn serialize(&self) -> Result<FormData> {
        let mut data = FormData::new(None);
        data.add_string("value1", self.input1.value());
        data.add_string("value2", self.input2.value());
        data.add_string("value3", self.input3.value());
        data.add_string("value4", self.input4.value());
        data.add_string("value5", self.input5.value());
        let indexes = self.word_indexes.value().unwrap();
        data.add_object("indexes", indexes)?;
        Ok(data)
    }
    async fn activate(&self) -> Result<()> {
        let mut indexes: Vec<u8> = vec![];
        let mut numbers = [0u8, 100];
        let mut rng = thread_rng();

        while indexes.len() < 5 {
            rng.fill_bytes(&mut numbers);
            for number in numbers {
                if number < 12 && !indexes.contains(&number) {
                    indexes.push(number);
                }

                if indexes.len() == 5 {
                    break;
                }
            }
        }

        indexes.sort();

        let mut field_index = 1;
        let indexes = indexes
            .iter()
            .map(|n| {
                let n = n + 1;
                let label = match n {
                    1 => "1st word".to_string(),
                    2 => "2nd word".to_string(),
                    3 => "3rd word".to_string(),
                    _ => format!("{n}th word"),
                };

                let _ = match field_index {
                    1 => self.input1.set_label(&i18n(&label)),
                    2 => self.input2.set_label(&i18n(&label)),
                    3 => self.input3.set_label(&i18n(&label)),
                    4 => self.input4.set_label(&i18n(&label)),
                    5 => self.input5.set_label(&i18n(&label)),
                    _ => Ok(()),
                };

                field_index += 1;

                n
            })
            .collect();

        self.word_indexes.set_value(Some(indexes))?;

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
        let password = self.password.value();
        let password_confirm = self.password_confirm.value();
        if !password.eq(&password_confirm) {
            return Err("Password dont match.".into());
        }
        data.add_string("password", password);
        //data.add_string("password_confirm", password_confirm);
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
    #[field(title = "Step [INDEX]/[STEPS]")]
    stages: FormStages,
}

#[async_trait_without_send]
impl FormHandler for ImportForm {
    async fn load(&self) -> Result<()> {
        self.stages
            .add_stage(Arc::new(ImportFormStage1::try_new()?))?;
        self.stages
            .add_stage(Arc::new(ImportFormStageConfirmingSeeds::try_new()?))?;
        self.stages
            .add_stage(Arc::new(ImportFormStage3::try_new()?))?;
        self.stages.activate_stage(0, Some(self.footer())).await?;
        Ok(())
    }

    async fn submit(&self) -> Result<()> {
        let data = match self.stages.serialize_stage().await {
            Ok(data) => data,
            Err(err) => {
                log_trace!("Form Serialize error: {err:?}");
                show_error(err.to_string().as_str())?;
                return Ok(());
            }
        };

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
