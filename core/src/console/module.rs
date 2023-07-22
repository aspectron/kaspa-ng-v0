use crate::prelude::*;
use kaspa_cli::{kaspa_cli, TerminalOptions, TerminalTarget};
use workflow_ux::{result::Result, view::View};

pub struct Menu {
    pub root: SectionMenu,
    pub group: MenuGroup,
    pub cli: MenuItem,
}

impl Menu {
    pub fn new() -> Result<Self> {
        let root = section_menu!(
            workspace().menu().main().default(),
            ("Console", "Console"),
            Icon::Console
        );
        let group = menu_group!(root, "Console");
        let cli = menu_item!(group, "Terminal", Icon::Console, Console::cli);

        Ok(Self { root, group, cli })
    }
}

#[derive(Module)]
pub struct Console {
    pub menu: Menu,
    cli_view: Arc<Mutex<Option<Arc<view::Html>>>>,
}

impl Console {
    pub fn new() -> Result<Self> {
        Ok(Self {
            menu: Menu::new()?,
            cli_view: Arc::new(Mutex::new(None)),
        })
    }
}

#[async_trait_without_send]
impl ModuleInterface for Console {
    async fn evict(
        self: Arc<Self>,
        container: &Arc<view::Container>,
        _view: Arc<dyn view::View>,
    ) -> Result<()> {
        container.element().remove_attribute("data-view")?;
        Ok(())
    }
    async fn main(self: Arc<Self>) -> Result<()> {
        Ok(())
    }
}

impl Console {
    async fn cli(self: Arc<Self>) -> crate::result::Result<()> {
        let main = workspace().main();
        main.swap_from().await?;
        let view_opt = self.cli_view.lock()?.clone();

        let view = match view_opt {
            Some(view) => view,
            None => {
                let view = view::Html::try_new(
                    Some(self.clone()),
                    html! {
                        <div class="terminal-container" @container></div>
                    }?,
                )?;
                view.element().class_list().add_1("terminal-view")?;

                let container = view.html().hooks().get("container").unwrap().clone();
                let options = TerminalOptions::new()
                    .with_prompt("$ ")
                    .with_element(TerminalTarget::Element(container));
                workflow_core::task::wasm::dispatch(async move {
                    kaspa_cli(options, None)
                        .await
                        .map_err(|e| {
                            log_trace!("wallet-cli error: {e:?}");
                        })
                        .ok();
                });

                *self.cli_view.lock()? = Some(view.clone());

                view
            }
        };

        main.swap_to(view).await?;
        main.element().set_attribute("data-view", "terminal")?;
        Ok(())
    }
}
