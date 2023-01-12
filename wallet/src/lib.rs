
use wasm_bindgen::prelude::*;
use workflow_log::log_trace;

mod address_type;
mod xkey;
mod xpub;
use address_type::AddressType;
use xkey::{HDWallet, Result, Prefix};


async fn test()->Result<()>{

    // Generate random Mnemonic using the default language (English)
    //let mnemonic = Mnemonic::random(&mut OsRng, Default::default());

    // Derive a BIP39 seed value using the given password
    //let seed = mnemonic.to_seed("password");

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
    let xpriv_str = "xprv9s21ZrQH143K4DoTUWmhygbsRQjAn1amZFxKpKtDtxabRpm9buGS5bbU4GuYDp7FtdReX5VbdGgoWwS7RuyWkV6aqYQUW6cX5McxWE8MN57";//xpriv.as_str();
    let hd_wallet = HDWallet::from_str(xpriv_str)?;
    let xpriv = hd_wallet.to_string();
    let xpub = hd_wallet.public_key().to_string(Prefix::XPUB);
    log_trace!("xpriv: {}", xpriv.as_str());
    log_trace!("xpub : {}", xpub);

    let mut receive_addresses : Vec<String>= Vec::new();
    let mut change_addresses : Vec<String>= Vec::new();
    for index in 0..10{
        let address = hd_wallet.derive_address(AddressType::Receive, index).await?;
        receive_addresses.push(address.into());
        let address = hd_wallet.derive_address(AddressType::Change, index).await?;
        change_addresses.push(address.into());
    }

    log_trace!("Receive addresses:");
    for (index, address)in receive_addresses.iter().enumerate(){
        log_trace!("#{index}: {}", address);
    }
    log_trace!("Change addresses:");
    for (index, address)in change_addresses.iter().enumerate(){
        log_trace!("#{index}: {}", address);
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

    println!("");

    Ok(())
}

/*
fn main() {
    let result = test();
    println!("result: {:?}", result);
}
*/


#[wasm_bindgen]
pub async fn test_addresses(){
    let result = test().await;
    log_trace!("result: {:?}", result);
}
