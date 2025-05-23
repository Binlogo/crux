pub mod app;

use std::sync::LazyLock;

use crux_core::bridge::Bridge;
pub use crux_core::{Core, Request};
pub use crux_http as http;
pub use crux_kv as key_value;
pub use crux_platform as platform;
pub use crux_time as time;

pub use app::*;

// TODO hide this plumbing

#[cfg(not(target_family = "wasm"))]
uniffi::include_scaffolding!("shared");

static CORE: LazyLock<Bridge<CatFacts>> = LazyLock::new(|| Bridge::new(Core::new()));

/// Ask the core to process an event
/// # Panics
/// If the core fails to process the event
#[cfg_attr(target_family = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[must_use]
pub fn process_event(data: &[u8]) -> Vec<u8> {
    match CORE.process_event(data) {
        Ok(effects) => effects,
        Err(e) => panic!("{e}"),
    }
}

/// Ask the core to handle a response
/// # Panics
/// If the core fails to handle the response
#[cfg_attr(target_family = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[must_use]
pub fn handle_response(id: u32, data: &[u8]) -> Vec<u8> {
    match CORE.handle_response(id, data) {
        Ok(effects) => effects,
        Err(e) => panic!("{e}"),
    }
}

/// Ask the core to render the view
/// # Panics
/// If the view cannot be serialized
#[cfg_attr(target_family = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
#[must_use]
pub fn view() -> Vec<u8> {
    match CORE.view() {
        Ok(view) => view,
        Err(e) => panic!("{e}"),
    }
}
