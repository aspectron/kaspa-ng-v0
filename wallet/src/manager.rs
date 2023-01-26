use crate::*;

pub struct WalletManager {}

impl WalletManager {
    #[allow(clippy::new_without_default)]
    /// WalletManager Constructor
    pub fn new() -> Self {
        Self {}
    }

    /// Open wallet from mnemonic
    pub async fn open_wallet(
        &self,
        encrypted_wallet: &str,
        password: &str,
        wallet_generation: WalletGeneration,
    ) -> Result<Arc<dyn WalletWrapper>> {
        let wallet: Arc<dyn WalletWrapper> = match wallet_generation {
            WalletGeneration::Gen0 => WalletGen0::open_wallet(encrypted_wallet, password).await?,
            WalletGeneration::Gen1 => WalletGen1::open_wallet(encrypted_wallet, password).await?,
        };

        Ok(wallet)
    }
}
