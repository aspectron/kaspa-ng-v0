use crate::prelude::*;
use workflow_ux::result::Result;
use workflow_ux::utils::markdown;

pub struct Menu {
    pub root : SectionMenu,
    pub group: MenuGroup,
    pub test: MenuItem
}

impl Menu {
    pub fn new() -> Result<Self> {
        let root = section_menu!(workspace().menu().main().default(), ("Test","Test"), Icon::Wip);
        let group = menu_group!(root, "Test");
        let test = menu_item!(group, "Form", Icon::Wip, Test::test_form);
        Ok(Self { root, group, test})
    }
}



#[derive(Module)]
pub struct Test {
    pub menu : Menu 
}

impl Test {
    pub fn new()->Result<Self> {
        Ok(Self{ menu : Menu::new()? })
    }
}

#[async_trait_without_send]
impl ModuleInterface for Test {

    async fn main(self : Arc<Self>) -> Result<()>{
        log_trace!("test:main");
        //self.menu.test.activate()?;
        Ok(())
    }
}

#[form(title="Test Form")]
struct TestForm{
    #[field(
        //qr_finder_color="#009688",
        qr_logo_size=20,
        qr_text="kaspa:qqs7fxw0ekhwef84auvydshrlrd3xlmvp8z2877uhhw42lwthafe70tglrjdg?amount=10000000",
        qr_logo="/resources/images/kaspa-180x180.png"
    )]
    qrcode:qr::QRCode
}
#[async_trait_without_send]
impl FormHandler for TestForm{
    async fn load(&self)->Result<()>{
        Ok(())
    }
    async fn submit(&self)->Result<()>{
        Ok(())
    }
}

impl Test {

    async fn news(self: Arc<Self>) -> Result<()>{
        let main = workspace().main();
        main.swap_from().await?;

        let view = view::Html::try_new(Some(self.clone()),
            html!{
                <h1 class="center">{i18n("News")}</h1>
                <div>{markdown(&i18n("

### 2022-11-01 Prototype/Demo release

"
))?}</div>
            }?
        )?;
        
        main.swap_to(view).await?;
        Ok(())
    }



    async fn welcome(self: Arc<Self>) -> Result<()>{
        let main = workspace().main();
        main.swap_from().await?;
        //let qrcode = qr::QRCode::new(pane, attributes, docs)

        let view = view::Html::try_new(Some(self.clone()),
            html!{
                <h1 class="center">{i18n("Welcome to Kaspa wallet")}</h1>
                <div>{markdown(&i18n(""))?}</div>

            }?
        )?;
        
        main.swap_to(view).await?;
        Ok(())
    }

    async fn test_form(self: Arc<Self>) -> Result<()>{
        let main = workspace().main();
        main.swap_from().await?;

        let view = TestForm::try_create_layout_view(
            Some(self.clone())
        ).await?;
        
        main.swap_to(view).await?;
        Ok(())
    }
}
