use crate::prelude::*;
use workflow_ux::result::Result;

pub struct Menu {
    pub root : SectionMenu,
    pub group: MenuGroup,
    pub send_and_receive: MenuItem,
    pub transactions: MenuItem,
    pub settings: MenuItem
}

impl Menu {
    pub fn new() -> Result<Self> {
        let root = section_menu!(workspace().menu().main().default(), ("Wallet","Wallet"), Icon::Wallet);
        let group = menu_group!(root,"Wallet");
        let send_and_receive = menu_item!(group, "Send & Receive", Icon::SendAndReceive, Wallet::send_and_receive);
        let transactions = menu_item!(group, "Transactions", Icon::Transactions, Wallet::transactions);
        let settings = menu_item!(group, "Settings", Icon::Settings, Wallet::settings);

        Ok(Self {
            root,
            group,
            send_and_receive,
            transactions,
            settings
        })
    }
}



#[derive(Module)]
pub struct Wallet {
    pub menu : Menu 
}

impl Wallet {
    pub fn new()->Result<Self> {
        Ok(Self{ menu : Menu::new()? })
    }
}

#[async_trait_without_send]
impl ModuleInterface for Wallet {

    async fn main(self : Arc<Self>) -> Result<()>{
        log_trace!("Wallet:main");
        self.menu.send_and_receive.activate()?;
        Ok(())
    }
}
/*
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
*/

impl Wallet {

    async fn send_and_receive(self: Arc<Self>) -> Result<()>{
        templates::under_development().await?;
        Ok(())
    }

    async fn transactions(self: Arc<Self>) -> Result<()>{
        templates::under_development().await?;
        Ok(())
    }

    async fn settings(self: Arc<Self>) -> Result<()>{
        templates::under_development().await?;
        Ok(())
    }
}
