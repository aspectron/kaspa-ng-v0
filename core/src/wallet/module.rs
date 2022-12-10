use crate::prelude::*;
use workflow_core::channel::Sender;
use workflow_ux::result::Result;
use workflow_ux::events::*;
use crate::application::Event;

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
    pub menu : Menu,
    wallet_view:Arc<Mutex<Option<Arc<WalletView>>>>,
}

impl Wallet {
    pub fn new()->Result<Self> {
        Ok(Self{
            menu : Menu::new()?,
            wallet_view:Arc::new(Mutex::new(None))
        })
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

#[derive(HtmlView)]
#[evict_handler="on_evict"] // <-- self.on_evict();
//#[evict_handler] //<-- self.evict_handler();
pub struct WalletView{
    event_sender:Arc<Mutex<Option<Sender<Event>>>>,
    html:Arc<Mutex<Option<Arc<view::Html>>>>
}

impl WalletView{
    fn new(module:Arc<Wallet>)->Result<Arc<Self>>{
        let view = Arc::new(Self{
            event_sender:Arc::new(Mutex::new(None)),
            html:Arc::new(Mutex::new(None))
        });

        view.clone().init(module)?;

        Ok(view)
    }

    fn init(self: Arc<Self>, module:Arc<Wallet>)->Result<()>{

        let address = "kaspa:qpzfe25efgnmgr482958dfrshf3t3fpu9gryf8227w0ll3kf5tqfzrtxs2507";
        let mut qr_options = qr::Options::default();
        qr_options.logo_size = 20;
        qr_options.logo = Some("/resources/images/kaspa-180x180.png".to_string());

        let qr_code = qr::QRCode::create(address, qr_options)?;


        let this = Arc::downgrade(&self);
        let this2 = Arc::downgrade(&self);

        let view = view::Html::try_new(Some(module), html!{
            <div class="wallet-view">
                <div class="balance-badge">
                    <div class="balance">
                        <span class="label">{i18n("Available")}</span>
                        <span class="value" @balance>"371,822.30358833 KAS"</span>
                    </div>
                    <div class="balance pending">
                        <span class="label-pending">{i18n("Pending")}</span>
                        <span class="value-pending">"0 KAS"</span>
                    </div>
                </div>
                <div class="address-badge">
                    <div>{i18n("Receive Address:")}</div>
                    <div class="address-holder">
                        <div class="address-input">{address}</div>
                        <div class="icon copy-address" icon={Icon::Copy} title={i18n("Copy to clipboard")}></div>
                    </div>
                </div>
                <div class="qr-code-holder">
                    {qr_code}
                    <div class="buttons-holder">
                        <flow-btn primary="true">{i18n("SEND")}</flow-btn>
                        <div class="sep"></div>
                        <flow-btn primary="true">{i18n("Scan QR code")}</flow-btn>
                    </div>
                </div>
                <div class="buttons-holder">
                    <div class="sep"></div>
                    <flow-btn primary="true" !click={
                        if let Some(c) = this.clone().upgrade(){
                            let _ = c.subscribe();
                        }
                    }>{i18n("Subscribe")}</flow-btn>
                    <flow-btn primary="true"!click={
                        if let Some(c) = this2.clone().upgrade(){
                            let _ = c.unsubscribe();
                        }
                    }>{i18n("Unsubscribe")}</flow-btn>
                </div>
                <div class="status">
                    <div><label>{i18n("Wallet Status:")}</label> {i18n("Online")}</div>
                    <div><label>{i18n("DAA score:")}</label> "33,996,663"</div>
                </div>
            </div>
        }?)?;

        *self.html.lock()? = Some(view);
        //self.subscribe()?;
        Ok(())
    }

    fn subscribe(self: Arc<Self>)->Result<()>{
        if self.event_sender.lock()?.as_ref().is_some(){    
            return Ok(());
        }

        let (id, sender, receiver) = Application::register_event_channel();
        *self.event_sender.lock()? = Some(sender);

        let this = Arc::downgrade(&self);
        let this2 = Arc::downgrade(&self);
        subscribe(receiver, move |event|->CallbackResult{
            let view = this.clone();
            Box::pin(async move {
                if let Some(c) = view.upgrade(){
                    c.digest_event(event).await
                }else{
                    Ok(false)
                }
            })
        }, move ||{
            Application::unregister_event_channel(id);
            if let Some(c) = this2.clone().upgrade(){
                *c.event_sender.lock().unwrap() = None;
            }
        })?;

        Ok(())
    }

    fn unsubscribe(&self)->Result<()>{
        log_info!("WalletView unsubscribe");

        if let Some(sender) = self.event_sender.lock()?.as_ref(){
            sender.try_send(Event::Halt)?;
        }

        *self.event_sender.lock()? = None;

        Ok(())
    }

    async fn digest_event(self: Arc<Self>, event:Event)->Result<bool>{
        log_info!("Wallet: got event: {:?}", event);
        match event {
            Event::Balance(balance)=>{
                let binding = self.html.lock()?;
                let html_ = binding.as_ref().unwrap().html();
                let hooks = html_.hooks();
                hooks.get("balance").unwrap().set_inner_html(&format!("{} KAS", balance));

            }
            Event::Halt=>{
                return Ok(false)
            }
        }
        Ok(true)
    }

    fn on_evict(&self)->Result<()>{
        println!("WalletView: on_evict:");
        self.unsubscribe()?;
        Ok(())
    }

}


impl Wallet {

    async fn send_and_receive(self: Arc<Self>) -> Result<()>{
        let main = workspace().main();
        main.swap_from().await?;
        let mut is_new = false;
        let view = match self.wallet_view.lock()?.as_ref(){
            Some(view)=>view.clone(),
            None=>{
                let view = WalletView::new(self.clone())?;
                is_new = true;
                view
            }
        };

        if is_new{
            *self.wallet_view.lock()? = Some(view.clone());
        }

        view.clone().subscribe()?;
        main.swap_to(view).await?;
       
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

