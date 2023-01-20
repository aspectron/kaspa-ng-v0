use crate::tests::*;
//use crate::*;

/*
//#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
pub async fn _test_addresses() {
    let now = workflow_core::time::Instant::now();
    let _result = _test_addresses_impl(true).await;
    log_trace!("address created in {}s", now.elapsed().as_secs());
    //log_trace!("result: {:?}", result);
}


#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
pub async fn _test_wallet_init()->Result<()>{

    let _result = _test_wallet_init_impl().await;
    log_trace!("init result {:?}", _result);
    println!("========================== init end ==========================> ");
    Ok(())
}
*/

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
pub async fn _start_tests() -> Result<()> {
    let _result = _start_tests_impl().await;
    Ok(())
}
