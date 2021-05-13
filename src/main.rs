#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;
use app::TestApp;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = app::TestApp::default();
    eframe::run_native(Box::new(app));
}
