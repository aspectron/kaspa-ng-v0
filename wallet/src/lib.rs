extern crate alloc;
// use std::time::Duration;
use wasm_bindgen::prelude::*;
use workflow_core::task::*;
use workflow_log::log_trace;
use zeroize::Zeroizing;

//use bip32;

mod private_key;
mod public_key;
mod xkey;
mod xprivate_key;
mod xpublic_key;

mod address_type;
mod attrs;
mod child_number;
mod error;
mod mnemonic;
mod prefix;
mod result;
mod types;
mod wallets;

pub use address_type::AddressType;
pub use addresses::{Address, Prefix as AddressPrefix};
pub use attrs::ExtendedKeyAttrs;
pub use child_number::ChildNumber;
pub use mnemonic::{Language, Mnemonic};
pub use prefix::Prefix;
pub use private_key::PrivateKey;
pub use public_key::PublicKey;
pub use types::*;
pub use wallets::HDWalletGen0;
pub use wallets::HDWalletGen1;
pub use xkey::ExtendedKey;
pub use xprivate_key::ExtendedPrivateKey;
pub use xpublic_key::ExtendedPublicKey;

pub trait SecretKeyExt {
    fn get_public_key(&self) -> secp256k1_ffi::PublicKey;
    fn as_str(&self, attrs: ExtendedKeyAttrs, prefix: Prefix) -> Zeroizing<String>;
}

impl SecretKeyExt for secp256k1_ffi::SecretKey {
    fn get_public_key(&self) -> secp256k1_ffi::PublicKey {
        secp256k1_ffi::PublicKey::from_secret_key_global(self)
    }
    fn as_str(&self, attrs: ExtendedKeyAttrs, prefix: Prefix) -> Zeroizing<String> {
        // Add leading `0` byte
        let mut key_bytes = [0u8; KEY_SIZE + 1];
        key_bytes[1..].copy_from_slice(&self.to_bytes());

        let key = ExtendedKey {
            prefix,
            attrs,
            key_bytes,
        };

        Zeroizing::new(key.to_string())
    }
}

async fn test(_use_yield: bool) -> Result<()> {
    // init_yield();

    //return Ok(());

    // Generate random Mnemonic using the default language (English)
    //let mnemonic = Mnemonic::random(&mut OsRng, Default::default());

    let mnemonic = Mnemonic::new(
        "foster view oak response abuse sister oyster vapor mind host helmet purpose cram daring average warfare ring shoe scare bag arrest over scorpion lab",
        Language::English
    )?;

    // Derive a BIP39 seed value using the given password
    let seed = mnemonic.to_seed("");

    //println!("seed: {}", hex::encode(seed.as_bytes()));

    let xprv = ExtendedPrivateKey::<secp256k1_ffi::SecretKey>::new(seed)?;

    //println!("private_key: {}", hex::encode(xprv.private_key().to_bytes()));
    let xpriv_str = xprv.to_string(Prefix::KPRV);
    let xpriv_str = xpriv_str.as_str();

    //println!("xpriv: {}", xpriv_str);
    //println!("xpriv should be : kprv5y2qurMHCsXYrNfU3GCihuwG3vMqFji7PZXajMEqyBkNh9UZUJgoHYBLTKu1eM4MvUtomcXPQ3Sw9HZ5ebbM4byoUciHo1zrPJBQfqpLorQ");
    //println!("xpub : {}", xprv.public_key().to_string(Some(Prefix::KPUB)));
    //println!("xpub should be : kpub2Hv8W2rbSwaLD6XJt93SSEe6WPaoHyrH684QMpm5pKdQTY1CDvQoiPuXvCCfXFBKjHZXLQPDASuB3JREdS1GVKLV1P2AB2TiXRPAKXgjwFX");
    /*
    let address_path = format!("44'/111111'/0'/0/{}", 1);
    let children = address_path.split("/");
    for child in children{
        let c = child.parse::<ChildNumber>()?;
        //key = key.derive_child(c.clone())?;
        //println!("\nc:    {c:?}");
        //println!("key: {:#?}", key2);
        xprv = xprv.derive_child(c)?;

        println!("\nkey: {:?}\npub: {}\nc: {c:?}",
            //key.to_string(Prefix::XPRV).as_str(),
            xprv.to_string(Prefix::KPRV).as_str(),
            xprv.public_key().to_string(Some(Prefix::KPUB))
        );
        //sleep(Duration::from_secs(0)).await;
        //yield_now().await
    }

    let pubkey = &xprv.private_key().get_public_key().to_bytes()[1..];
    //let pubkey = &private_key.public_key().to_bytes()[1..];
    let address = Address{
        prefix: AddressPrefix::Mainnet,
        version: 0,
        payload: pubkey.to_vec()
    };
    let address_str: String = address.into();
    println!("\naddress 1 : {}", address_str);
    println!("address =>: kaspa:qzn3qjzf2nzyd3zj303nk4sgv0aae42v3ufutk5xsxckfels57dxjjed4qvlx");
    print!("\n\n==================\n");

    //Ok(())
    */
    // Derive the root `XPrv` from the `seed` value
    //let root_xprv = XPrv::new(&seed)?;
    //assert_eq!(root_xprv, XPrv::derive_from_path(&seed, &"m".parse()?)?);

    // Derive a child `XPrv` using the provided BIP32 derivation path
    //let child_path = "m/0/2147483647'/1/2147483646'";
    //let child_xprv = XPrv::derive_from_path(&seed, &child_path.parse()?)?;

    // Get the `XPub` associated with `child_xprv`.
    //let child_xpub = child_xprv.public_key();

    //let xpriv = child_xprv.to_string(Prefix::XPRV);
    //let xpub = child_xpub.to_string(Prefix::XPUB);
    //let xpriv_str = "xprv9s21ZrQH143K4DoTUWmhygbsRQjAn1amZFxKpKtDtxabRpm9buGS5bbU4GuYDp7FtdReX5VbdGgoWwS7RuyWkV6aqYQUW6cX5McxWE8MN57"; //xpriv.as_str();

    let hd_wallet = HDWalletGen1::from_str(xpriv_str).await?;
    //let xpub = hd_wallet.public_key().to_string(Some(Prefix::KPUB));
    //log_trace!("\nmasterKey : {}", hd_wallet.to_string().as_str());
    //log_trace!("masterPubKey : {}", xpub);

    //log_trace!("\nextendedKey: {}", hd_wallet.receive_wallet().to_string(Prefix::KPRV).as_str());
    //log_trace!("extendedPubKey: {}\n", hd_wallet.receive_wallet().public_key().to_string(Some(Prefix::KPUB)));

    let mut receive_addresses: Vec<String> = Vec::new();
    let mut change_addresses: Vec<String> = Vec::new();
    for index in 0..5000 {
        receive_addresses.push(hd_wallet.derive_receive_address(index).await?.into());
        change_addresses.push(hd_wallet.derive_change_address(index).await?.into());
        if _use_yield && index % 10 == 0 {
            yield_executor().await;
        }

        if index % 200 == 0 {
            log_trace!("generating index:{}", index);
        }
        //sleep(Duration::from_secs(1)).await;
    }

    log_trace!("Receive addresses:");
    for (index, address) in receive_addresses.iter().enumerate() {
        if index < 10 || index % 100 == 0 {
            log_trace!("#{index}: {}", address);
        }
    }
    log_trace!("Change addresses:");
    for (index, address) in change_addresses.iter().enumerate() {
        if index < 10 || index % 100 == 0 {
            log_trace!("#{index}: {}", address);
        }
    }

    //println!("key: {:?}", key.to_string(Prefix::XPRV).as_str());
    //let priv_key = xpriv_key. (child_number)

    /*
    // Serialize `child_xprv` as a string with the `xprv` prefix.
    let child_xprv_str = child_xprv.to_string(Prefix::XPRV);
    assert!(child_xprv_str.starts_with("xprv"));

    // Serialize `child_xpub` as a string with the `xpub` prefix.
    let child_xpub_str = child_xpub.to_string(Prefix::XPUB);
    assert!(child_xpub_str.starts_with("xpub"));

    // Get the ECDSA/secp256k1 signing and verification keys for the xprv and xpub
    let signing_key = child_xprv.private_key();
    let verification_key = child_xpub.public_key();

    // Sign and verify an example message using the derived keys.
    use bip32::secp256k1::ecdsa::{
        signature::{Signer, Verifier},
        Signature
    };

    let example_msg = b"Hello, world!";
    let signature: Signature = signing_key.sign(example_msg);
    assert!(verification_key.verify(example_msg, &signature).is_ok());
    */

    println!();

    Ok(())
}

/*
#[async_std::main]
async fn main() {
    let result = test().await;
    println!("result: {:?}", result);
}
*/

#[wasm_bindgen]
pub async fn test_addresses() {
    //spawn(async move {
    log_trace!("testing addresses");
    let result = test(true).await;
    log_trace!("result: {:?}", result);
    //});
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    pub async fn addresses() {
        let now = workflow_core::time::Instant::now();
        let _result = test(true).await;
        log_trace!("address created in {}s", now.elapsed().as_secs());
        //log_trace!("result: {:?}", result);
    }
}
