// use crate::application::Event;
use crate::prelude::*;
use workflow_ux::result::Result;

pub struct Menu {
    pub root: SectionMenu,
    pub group: MenuGroup,
    // pub test: MenuItem,
}

impl Menu {
    pub fn new() -> Result<Self> {
        let root = section_menu!(
            workspace().menu().main().default(),
            ("Test", "Test"),
            Icon::Wip
        );
        let group = menu_group!(root, "Test");
        // let test = menu_item!(group, "Form", Icon::Wip, Test::test_form);
        // Ok(Self { root, group, test })
        Ok(Self { root, group })
    }
}

// #[derive(Module)]
// pub struct Test {
//     pub menu: Menu,
//     _handle: workflow_wasm::timers::IntervalHandle,
// }

// impl Test {
//     pub fn new() -> Result<Self> {
//         //let mut count = 0;
//         let cl = Closure::<dyn FnMut()>::new(move || {
//             //log_trace!("set_interval");
//             //count += 1;
//             //if count == 20{
//             //    application().reflect(Event::Halt);
//             //workflow_wasm::timers::clear_interval(_handle);
//             //}else if count < 20{
//             application().reflect(Event::Balance(Id::new()));

//             application().reflect(Event::Blocks(Id::new().to_bytes()[0] as u32));
//             //}
//         });
//         let _handle = workflow_wasm::timers::set_interval(&cl, 500)?;
//         log_trace!("set_interval handle: {:?}", _handle);
//         cl.forget();

//         Ok(Self {
//             menu: Menu::new()?,
//             _handle,
//         })
//     }
// }

// #[async_trait_without_send]
// impl ModuleInterface for Test {
//     async fn main(self: Arc<Self>) -> Result<()> {
//         log_trace!("test:main");

//         Ok(())
//     }
// }

#[form(title = "Test Form")]
struct TestForm {
    #[field(
        //qr_finder_color="#009688",
        qr_logo_size=20,
        qr_text="kaspa:qqs7fxw0ekhwef84auvydshrlrd3xlmvp8z2877uhhw42lwthafe70tglrjdg?amount=10000000",
        qr_logo="/resources/images/kaspa-180x180.png"
    )]
    qrcode: qr::QRCode,
}
#[async_trait_without_send]
impl FormHandler for TestForm {
    async fn load(&self) -> Result<()> {
        Ok(())
    }
    async fn submit(&self) -> Result<()> {
        Ok(())
    }
}

// impl Test {
//     async fn test_form(self: Arc<Self>) -> Result<()> {
//         let main = workspace().main();
//         main.swap_from().await?;

//         let view = TestForm::try_create_layout_view(Some(self.clone())).await?;

//         main.swap_to(view).await?;
//         Ok(())
//     }
// }
