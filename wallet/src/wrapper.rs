use crate::*;
use workflow_async_trait::async_trait;

#[async_trait]
pub trait WalletWrapper {
    async fn open_wallet(encrypted_wallet: &str, password: &str) -> Result<Arc<Self>>
    where
        Self: Sized;

    async fn sync(&self) -> Result<()>;

    async fn receive_address(&self) -> Result<Address>;
}
