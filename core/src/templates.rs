use workflow_html::{html,Html};
use kaspa_core::prelude::*;
use kaspa_core::result::Result;

pub fn loading(msg:&str)->Result<Html>{
    Ok(html!{
        <div class="message-view loading-view">
            <h2 class="title center">{i18n(msg)}</h2>
            <div class="msg">
                <div>"Please standby... "</div>
                <img class="loading-img" src="/resources/images/loading.svg" />
            </div>
        </div>
    }?)
}
pub async fn under_development()->Result<()>{
    let main = workspace().main();
    main.swap_from().await?;

    let view = view::Html::try_new(None, html!{
        <div class="under-development center">
            <img src="/resources/images/wip.svg" />
            <h3 style="opacity:0.5;">"THIS FEATURE IS UNDER DEVELOPMENT"</h3>
        </div>
    }?)?;

    main.swap_to(view).await?;
    Ok(())
}
