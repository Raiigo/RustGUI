#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;
use app::TestApp;
use async_std;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
#[async_std::main]
async fn main() -> surf::Result<()> {
    // let app = app::TestApp::default();
    // eframe::run_native(Box::new(app));


    let mut res = surf::get("https://discord.com/api/v9/users/@me").header("Authorization", "Bot NzkxNjM2MjQyNDE2MDA5MjU2.X-SCtA.a03ZiwCIfWck2vrF2gGfy6KfVVE").await?;
    println!("{}", res.body_string().await?);
    Ok(()) 
    
}