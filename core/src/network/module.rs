use crate::network::view::*;
use crate::prelude::*;
use workflow_ux::result::Result;

pub struct Menu {
    pub root: SectionMenu,
    pub group: MenuGroup,
    pub node: MenuItem,
    pub mining: MenuItem,
    pub settings: MenuItem,
}

impl Menu {
    pub fn new() -> Result<Self> {
        let root = section_menu!(
            workspace().menu().main().default(),
            ("Network", "Network"),
            Icon::Network
        );
        let group = menu_group!(root, "Network");
        let node = menu_item!(group, "Node", Icon::Wip, Network::node);
        let mining = menu_item!(group, "Mining", Icon::Wip, Network::mining);
        let settings = menu_item!(group, "Settings", Icon::Settings, Network::settings);

        Ok(Self {
            root,
            group,
            node,
            mining,
            settings,
        })
    }
}

#[derive(Module)]
pub struct Network {
    pub menu: Menu,
    node_view: Arc<Mutex<Option<Arc<NodeView>>>>,
}

impl Network {
    pub fn new() -> Result<Self> {
        Ok(Self {
            menu: Menu::new()?,
            node_view: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait_without_send]
impl ModuleInterface for Network {
    async fn main(self: Arc<Self>) -> Result<()> {
        Ok(())
    }
}

impl Network {
    async fn node(self: Arc<Self>) -> Result<()> {
        let main = workspace().main();
        main.swap_from().await?;

        let view = self.node_view.lock()?.clone();
        let view = match view {
            Some(view) => view,
            None => {
                //let view = NodeView::new(self.clone())?;
                //*self.node_view.lock()? = Some(view.clone());
                NodeView::new(self.clone())?
            }
        };

        view.clone().subscribe()?;
        main.swap_to(view).await?;
        Ok(())
    }

    async fn mining(self: Arc<Self>) -> Result<()> {
        templates::under_development().await?;
        Ok(())
    }

    async fn settings(self: Arc<Self>) -> Result<()> {
        templates::under_development().await?;
        Ok(())
    }
}
