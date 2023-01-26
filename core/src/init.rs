use crate::prelude::*;
use crate::result::Result;
use wasm_bindgen::prelude::*;
use workflow_ux::wasm::{self, init_ux, load_component};

#[macro_export]
macro_rules! load {
    ($exp:expr, $name:literal, $cmp:literal) => {
        load_component(&$exp, $name, include_str!($cmp))?;
    };
}

fn load_components(flow_ux_path: String) -> Result<()> {
    load!(flow_ux_path, "flow-config.js", "components/config.js");
    wasm::load_components(&flow_ux_path)?;

    load!(flow_ux_path, "app.js", "components/app.js");
    Ok(())
}

#[wasm_bindgen(js_name = "initialize")]
pub fn initialize(workflow: &JsValue, modules: &JsValue) -> Result<()> {
    let win = window();
    let loc = win.location();
    let origin = loc.origin()?;
    load_components(origin + "/static/flow-ux.js")?;

    init_ux(workflow, modules)?;
    Ok(())
}
