use crate::prelude::*;
use workflow_ux::result::Result;

pub struct Menu {
    pub root : SectionMenu,
    pub group: MenuGroup,
    pub status: MenuItem
}

impl Menu {
    pub fn new() -> Result<Self> {
        let root = section_menu!(workspace().menu().main().default(), ("Status","Status"), Icon::Status);
        let group = menu_group!(root,"Status");
        let status = menu_item!(group, "Network & Process", Icon::Wip, Status::status);

        Ok(Self {
            root,
            group,
            status
        })
    }
}



#[derive(Module)]
pub struct Status {
    pub menu : Menu 
}

impl Status {
    pub fn new()->Result<Self> {
        Ok(Self{ menu : Menu::new()? })
    }
}

#[async_trait_without_send]
impl ModuleInterface for Status {

    async fn main(self : Arc<Self>) -> Result<()>{
        Ok(())
    }
}

impl Status {

    async fn status(self: Arc<Self>) -> Result<()>{
        templates::under_development().await?;
        Ok(())
    }
}
