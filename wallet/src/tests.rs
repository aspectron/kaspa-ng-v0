use crate::*;

//#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
pub async fn _addresses() {
    let now = workflow_core::time::Instant::now();
    let _result = test(true).await;
    log_trace!("address created in {}s", now.elapsed().as_secs());
    //log_trace!("result: {:?}", result);
}


#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
pub async fn init()->Result<()>{

    let manager = WalletManager::new();

    let wallet_str = "";
    let password = "";

    let wallet = manager.open_wallet(wallet_str, password, WalletGeneration::Gen0).await?;

    wallet.sync().await?;
    let address = wallet.receive_address().await?;
    println!("receive_address: {}", address);

    Ok(())
}

