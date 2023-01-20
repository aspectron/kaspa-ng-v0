use crate::tests::*;
use crate::*;

#[wasm_bindgen]
pub async fn start_tests() {
    let r = _start_tests_impl().await;
    log_trace!("start_tests: result {:?}", r);
}
/*
#[wasm_bindgen]
pub async fn test_addresses() {
    log_trace!("testing addresses");
    let result = _test_addresses_impl(true).await;
    log_trace!("result: {:?}", result);
}

#[wasm_bindgen]
pub async fn test_wallet_init() {
    log_trace!("testing init");
    let result = _test_wallet_init_impl().await;
    log_trace!("result: {:?}", result);
}
*/
