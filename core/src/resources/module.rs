use crate::prelude::*;
use workflow_ux::result::Result;

pub struct Menu {
    pub root: SectionMenu,
    pub group: MenuGroup,
    pub community: MenuItem,
    pub donations: MenuItem,
    pub credits: MenuItem,
}

impl Menu {
    pub fn new() -> Result<Self> {
        let root = section_menu!(
            workspace().menu().main().default(),
            ("Resources", "Resources"),
            Icon::Resources
        );
        let group = menu_group!(root, "Resources");
        let community = menu_item!(group, "Community", Icon::Wip, Resources::community);
        let donations = menu_item!(group, "Donations", Icon::Wip, Resources::donations);
        let credits = menu_item!(group, "Credits", Icon::Credits, Resources::credits);

        Ok(Self {
            root,
            group,
            community,
            donations,
            credits,
        })
    }
}

#[derive(Module)]
pub struct Resources {
    pub menu: Menu,
}

impl Resources {
    pub fn new() -> Result<Self> {
        Ok(Self { menu: Menu::new()? })
    }
}

#[async_trait_without_send]
impl ModuleInterface for Resources {
    async fn main(self: Arc<Self>) -> Result<()> {
        Ok(())
    }
}

impl Resources {
    async fn community(self: Arc<Self>) -> Result<()> {
        templates::under_development().await?;
        Ok(())
    }

    async fn donations(self: Arc<Self>) -> Result<()> {
        templates::under_development().await?;
        Ok(())
    }

    async fn credits(self: Arc<Self>) -> Result<()> {
        templates::under_development().await?;
        Ok(())
    }
}
