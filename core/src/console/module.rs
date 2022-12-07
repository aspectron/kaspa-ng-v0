use crate::prelude::*;
use workflow_ux::result::Result;

pub struct Menu {
    pub root : SectionMenu,
    pub group: MenuGroup,
    pub cli: MenuItem,
}

impl Menu {
    pub fn new() -> Result<Self> {
        let root = section_menu!(workspace().menu().main().default(), ("Console","Console"), Icon::Console);
        let group = menu_group!(root,"Console");
        let cli = menu_item!(group, "Terminal", Icon::Console, Console::cli);
        
        Ok(Self {
            root,
            group,
            cli
        })
    }
}



#[derive(Module)]
pub struct Console {
    pub menu : Menu 
}

impl Console {
    pub fn new()->Result<Self> {
        Ok(Self{ menu : Menu::new()? })
    }
}

#[async_trait_without_send]
impl ModuleInterface for Console {

    async fn main(self : Arc<Self>) -> Result<()>{
        Ok(())
    }
}

impl Console {

    async fn cli(self: Arc<Self>) -> Result<()>{
        templates::under_development().await?;
        Ok(())
    }
}
