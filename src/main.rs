#[cfg(not(target_arch = "wasm32"))]
use workflow_http::error::Error;

#[cfg(not(target_arch = "wasm32"))]
#[async_std::main]
async fn main() -> Result<(), Error> {
    use duct::cmd;
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use std::path::Path;
    use workflow_http::{stores, BasicAuthenticator, Router};

    // ~~~

    if !Path::new("./node_modules").exists() {
        println!("\n\nnode_modules folder is absent... running npm install...\n");
        cmd!("npm", "install").run()?;
    }

    if !Path::new("./root/kaspa").exists() {
        println!("\n\nkaspa wasm folder is absent... running wasm-pack...\n");
        cmd!(
            "wasm-pack",
            "build",
            "--target",
            "web",
            "--out-name",
            "kaspa",
            "--out-dir",
            "../root/kaspa",
            "--features",
            "test"
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
    let overrides = BTreeMap::from([]);

    let router = Router::new_with_overrides(cwd.clone(), mount_map, source_map, overrides);

    let tide_secret: &[u8] = &(0..64).map(|_| rand::random::<u8>()).collect::<Vec<u8>>();

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
        tide_secret, /*std::env::var("TIDE_SECRET")
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
fn main() -> std::result::Result<(), String> {
    panic!("wasm32 target is not supported");
}
