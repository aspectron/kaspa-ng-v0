use web_sys::Element;
use workflow_core::enums::*;
use convert_case::{Case, Casing};

#[describe_enum]
#[derive(Debug)]
pub enum Icon {
    Default,
    Work,
    // ~
    Kaspa,
    Profile,
    Identity,
    FingerprintScan,
    User,
    // ~
    Certificate,
    Classroom,
    License,
    // ~
    Settings,
    Console,
    Connected,
    Wallet,
    SendAndReceive,
    Transactions,
    Help,
    Network,
    Resources,
    Credits,
    Status,

    Copy,

    Ban,
    Clock,

    Home1,
    Home2,
    News,
    Close,
    Cross,
    Wip
}

impl Icon{
    pub fn element(self)->workflow_ux::result::Result<Element>{
        let icon: workflow_ux::icon::Icon = self.into();
        Ok(icon.element()?)
    }
}

impl Into<workflow_ux::icon::Icon> for Icon {
    fn into(self) -> workflow_ux::icon::Icon {
        let name = self.as_str()
            .from_case(Case::Camel)
            .to_case(Case::Kebab);
        workflow_ux::icon::Icon::css(name)
    }
}

impl From<Icon> for String{
    fn from(icon:Icon) -> Self {
        let icon:workflow_ux::icon::Icon = icon.into();
        icon.to_string()
    }
}