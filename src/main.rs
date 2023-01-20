#[cfg(not(target_arch = "wasm32"))]
use workflow_http::error::Error;

//#[cfg(not(target_arch = "wasm32"))]
//mod builder;

// #[cfg(not(target_arch = "wasm32"))]
// mod tokens;

#[cfg(not(target_arch = "wasm32"))]
#[async_std::main]
async fn main() -> Result<(), Error> {
    use std::collections::HashMap;
    //use std::io::Error;
    //use kaizen::result::Result;
    use workflow_http::{BasicAuthenticator, stores, Router};
    //use flow_server::error::*;
    use std::{fs, io, collections::BTreeMap};
    use std::path::Path;
    use duct::cmd;

    // ~~~

    if Path::new("./node_modules").exists() != true {
        println!("\n\nnode_modules folder is absent... running npm install...\n");
        cmd!("npm","install").run()?;
    }

    if Path::new("./root/workflow").exists() != true {
        println!("\n\nworkflow wasm folder is absent... running wasm-pack...\n");
        cmd!(
            "wasm-pack",
            "build",
            "--target",
            "web",
            "--out-name",
            "workflow",
            "--out-dir",
            "../root/workflow",
            "--features","test"
        )
        .dir("wasm")
        .run()?;
    }

    // ~~~

    tide::log::start();

    let cwd = std::env::current_dir().unwrap();
    let mut mount_map = HashMap::new();
    mount_map.insert("flow-ux", "/flow-ux");
    mount_map.insert("lib", "/lib");
    mount_map.insert("workflow", "/workflow");
    //mount_map.insert("solflare-wallet-web", "/node_modules/@solflare-wallet/sdk/lib/esm/adapters/web/");
    mount_map.insert("node_modules", "/node_modules");
    let mut source_map = HashMap::new();
    source_map.insert("lib", "/root/lib");
    source_map.insert("workflow", "/root/workflow");
    //source_map.insert("solflare-wallet-web", "/node_modules/@solflare-wallet/sdk/lib/esm/adapters/");
    source_map.insert("node_modules", "/node_modules");
    let mut overrides = BTreeMap::from([
        (
            "eventemitter3".to_string(),
            //"/node_modules/eventemitter3/umd/eventemitter3.js".to_string()
            "/lib/esm/eventemitter3.js".to_string(),
        ),
        (
            "@solana/web3.js".to_string(),
            //"/node_modules/@solana/web3.js/lib/index.iife.js".to_string(),
            "/lib/esm/solana.js".to_string(),
        ),
        (
            "@solflare-wallet/sdk".to_string(),
            "/lib/esm/solflare-sdk.js".to_string()
        ),
        /*(
            "@solflare-wallet/sdk".to_string(),
            "/node_modules/@solflare-wallet/sdk/lib/esm/index.js".to_string()
        ),
        (
            "./adapters/web".to_string(),
            "./adapters/web.js".to_string()
        ),
        (
            "./adapters/iframe".to_string(),
            "./adapters/iframe.js".to_string()
        ),
        (
            "./base".to_string(),
            "./base.js".to_string()
        ),
        (
            "bs58".to_string(),
            "/lib/esm/bs58.js".to_string()
        ),
        (
            "uuid".to_string(),
            "/node_modules/uuid/wrapper.mjs".to_string()
        ),
        (
            "@project-serum/sol-wallet-adapter".to_string(),
            "/node_modules/@project-serum/sol-wallet-adapter/dist/esm/index.js".to_string()
        )*/
        /*(
            "@ledgerhq/hw-transport".to_string(),
            "/node_modules/@ledgerhq/hw-transport/lib-es/Transport.js".to_string()
        ),
        (
            "@ledgerhq/errors".to_string(),
            "/node_modules/@ledgerhq/errors/lib-es/index.js".to_string()
        ),
        (
            "tweetnacl".to_string(),
            "/node_modules/tweetnacl/nacl-fast.js".to_string()
        ),
        (
            "@solana/buffer-layout".to_string(),
            "/node_modules/@solana/buffer-layout/lib/Layout.js".to_string()
        )*/
    ]);
    let adapters = fs::read_dir("./node_modules/@solana")
        .map_err(|e|{ panic!("\n\nError reading './node_modules/@solana'...\n{}\n\nDid you run npm install?\n\n",e.to_string()); })
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<std::result::Result<Vec<_>, io::Error>>()?;
    //println!("adapters: {:#?}", adapters);

    for adapter in adapters{
        let path = adapter.as_path();
        //println!("path: {:?}", path);
        if let Some(s) = path.to_str(){
            if !s.contains(&"wallet-adapter-"){
                continue;
            }
            if let Some(file_name) = path.file_name(){
                if let Some(name) = file_name.to_str(){
                    //println!("name: {}", &name);
                    if name.eq("wallet-adapter-solflare"){
                        overrides.insert(
                            format!("@solana/{}", name),
                            format!("/lib/esm/solflare.js"),
                        );
                    }else{
                        overrides.insert(
                            format!("@solana/{}", name),
                            format!("/node_modules/@solana/{}/lib/esm/index.js", name),
                        );
                    }
                }
            }
        }
    }

    let router = Router::new_with_overrides(
        cwd.clone(),
        mount_map,
        source_map,
        overrides
    );

    let tide_secret :&[u8] = &(0..64).map(|_| { rand::random::<u8>() }).collect::<Vec<u8>>();

    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    let file = cwd.as_path().join(".auth");
    if file.exists() {
        let memory_store = stores::from_hjson_file(file.as_path())?;
        let authenticator = BasicAuthenticator::new(memory_store);
        authenticator.init(&mut app);
    }

    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        tide_secret
        /*std::env::var("TIDE_SECRET")
            .expect(
                "Please provide a TIDE_SECRET value of at \
                      least 32 bytes in order to run this example",
            )
            .as_bytes(),*/
    ));

    app.with(tide::utils::Before(
        |mut request: tide::Request<()>| async move {
            let session = request.session_mut();
            let visits: usize = session.get("visits").unwrap_or_default();
            session.insert("visits", visits + 1).unwrap();
            request
        },
    ));

    app.at("/").serve_file("root/index.html")?;

    app.at("/reset")
        .get(|mut req: tide::Request<()>| async move {
            req.session_mut().destroy();
            Ok(tide::Redirect::new("/"))
        });

    router.init(&mut app);
    app.at("/").serve_dir("root/")?;
    app.at("/node_modules").serve_dir("node_modules/")?;

    
    app.listen("0.0.0.0:8080").await?;

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() -> std::result::Result<(),String> {
    panic!("wasm32 target is not supported");
}