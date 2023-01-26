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

/*
#[form(title="Import Wallet")]
struct ImportForm{
    #[field()]
    mnemonic: Mnemonic,

    stage: Hidden
}
*/

pub struct ImportForm {
    _layout: ElementLayout,
    pub _footer: workflow_ux::form_footer::FormFooter,
    mnemonic: Mnemonic,
    stage: Hidden,
}
unsafe impl Send for ImportForm {}

unsafe impl Sync for ImportForm {}

impl ImportForm {
    pub fn show(&self, show: bool) -> workflow_ux::result::Result<()> {
        let el = self._layout.element();
        if show {
            Ok(el.remove_attribute("hidden")?)
        } else {
            Ok(el.set_attribute("hidden", "true")?)
        }
    }
    pub async fn try_create_layout_view(
        module: Option<std::sync::Arc<dyn workflow_ux::module::ModuleInterface>>,
    ) -> workflow_ux::result::Result<std::sync::Arc<workflow_ux::view::Layout<Self, ()>>> {
        Ok(Self::try_create_layout_view_with_data(module, Option::<()>::None).await?)
    }
    pub async fn try_create_layout_view_with_data<D: Send + 'static>(
        module: Option<std::sync::Arc<dyn workflow_ux::module::ModuleInterface>>,
        data: Option<D>,
    ) -> workflow_ux::result::Result<std::sync::Arc<workflow_ux::view::Layout<Self, D>>> {
        let el = workflow_ux::document().create_element("div")?;
        let mut layout = Self::try_inject(&el)?;
        layout.load().await?;
        let view = workflow_ux::view::Layout::try_new(module, layout, data)?;
        {
            let layout_clone = view.layout();
            let mut locked = layout_clone.lock().expect(&format!(
                "Unable to lock form {} for footer binding.",
                "ImportForm"
            ));
            locked
                ._footer
                .bind_layout("ImportForm".to_string(), view.clone())?;
        }
        Ok(view)
    }
    pub fn try_new() -> workflow_ux::result::Result<ImportForm> {
        let el = workflow_ux::document().create_element("div")?;
        let layout = Self::try_inject(&el)?;
        Ok(layout)
    }
    pub fn try_inject(parent: &web_sys::Element) -> workflow_ux::result::Result<ImportForm> {
        let root =
            ElementLayout::try_inject(parent, workflow_ux::layout::ElementLayoutStyle::Form)?;
        let attributes = Attributes::new();
        let docs = Docs::new();
        Ok(ImportForm::new(&root, &attributes, &docs)?)
    }
    pub fn new(
        parent_layout: &ElementLayout,
        attributes: &Attributes,
        docs: &Docs,
    ) -> workflow_ux::result::Result<ImportForm> {
        let attr_list: Vec<(String, String)> =
            vec![("title".to_string(), "Import Wallet".to_string())];
        let mut attributes = Attributes::new();
        for (k, v) in attr_list.iter() {
            attributes.insert(k.to_string(), v.clone());
        }
        let layout_style = workflow_ux::layout::ElementLayoutStyle::Form;
        let _layout = ElementLayout::new(parent_layout, layout_style, &attributes)?;
        let mnemonic = {
            let mut ctl_attributes = Attributes::new();
            let ctl_attr_list: Vec<(String, String)> = vec![];
            for (k, v) in ctl_attr_list.iter() {
                ctl_attributes.insert(k.to_string(), v.clone());
            }
            let mut layout_attributes = Attributes::new();
            let layout_attr_list: Vec<(String, String)> = vec![];
            for (k, v) in layout_attr_list.iter() {
                layout_attributes.insert(k.to_string(), v.clone());
            }
            let docs: Vec<&str> = vec![];
            let mnemonic = Mnemonic::new(&_layout, &ctl_attributes, &docs)?;
            let child = mnemonic.element();
            _layout.append_child(&child, &layout_attributes, &docs)?;
            mnemonic
        };
        let stage = {
            let mut ctl_attributes = Attributes::new();
            let ctl_attr_list: Vec<(String, String)> = vec![];
            for (k, v) in ctl_attr_list.iter() {
                ctl_attributes.insert(k.to_string(), v.clone());
            }
            let mut layout_attributes = Attributes::new();
            let layout_attr_list: Vec<(String, String)> = vec![];
            for (k, v) in layout_attr_list.iter() {
                layout_attributes.insert(k.to_string(), v.clone());
            }
            let docs: Vec<&str> = vec![];
            let stage = Hidden::new(&_layout, &ctl_attributes, &docs)?;
            //let child = stage.element();
            //_layout.append_child(&child, &layout_attributes, &docs)?;
            stage
        };
        let mut _footer = {
            let layout_attributes = Attributes::new();
            let ctl_attributes = Attributes::new();
            let docs: Vec<&str> = vec![];
            let footer =
                workflow_ux::form_footer::FormFooter::new(&_layout, &ctl_attributes, &docs)?;
            let child = footer.element();
            _layout.append_child(&child, &layout_attributes, &docs)?;
            footer
        };
        let layout = ImportForm {
            _layout,
            mnemonic,
            stage,
            _footer,
        };
        layout.init()?;
        Ok(layout)
    }
    pub fn layout(&self) -> ElementLayout {
        self._layout.clone()
    }
    pub fn set_submit_btn_text<T: Into<String>>(&self, text: T) -> workflow_ux::result::Result<()> {
        self._footer.set_submit_btn_text(text)?;
        Ok(())
    }
}
impl workflow_ux::layout::DefaultFunctions for ImportForm {}

impl workflow_ux::layout::Elemental for ImportForm {
    fn element(&self) -> web_sys::Element {
        self._layout.element()
    }
}
impl Clone for ImportForm {
    fn clone(&self) -> ImportForm {
        ImportForm {
            _layout: self._layout.clone(),
            mnemonic: self.mnemonic.clone(),
            stage: self.stage.clone(),
            _footer: self._footer.clone(),
        }
    }
}
impl Into<Element> for ImportForm {
    fn into(self) -> Element {
        self._layout.element()
    }
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
