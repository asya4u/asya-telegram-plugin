use lazy_static::lazy_static;
use std::ffi::{c_char, CString};
use tokio::runtime::Runtime;

use plugin_interface::{ApiCallbacksMap, EventState, PluginInfoCallback, PluginInformation, State};
use serde::Deserialize;

mod config;
mod telegram;
mod things;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

#[no_mangle]
pub static plugin_info: PluginInfoCallback = plugin_information;

#[no_mangle]
pub extern "C" fn plugin_information() -> *const PluginInformation {
    let plugin_name = CString::new("asya_telegram").unwrap();
    let name = plugin_name.into_raw().cast_const();

    let plugin_information = PluginInformation {
        name,
        init_callback: init,
        event_callback: handler,
        execute_callback: execute,
    };

    Box::into_raw(Box::new(plugin_information)).cast_const()
}

#[derive(Debug, Deserialize, Clone)]
struct AsyaResponse {
    pub message: String,
}

#[no_mangle]
pub extern "C" fn init(config: *const c_char, api: ApiCallbacksMap) -> *mut State {
    let config = things::extract_config(config);
    dbg!(&api);
    unsafe {
        let callback_ptr = api.callback("subscribe_to_events");
        if !callback_ptr.is_null() {
            let subscribe_fn: unsafe extern "C" fn(unsafe extern "C" fn(*const c_char)) =
                std::mem::transmute(callback_ptr);
            subscribe_fn(events_handler);
            println!("Subscribed to events");
        } else {
            println!("Failed to subscribe to events");
        }
    }
    telegram::run_tgbot(api, config);
    Box::into_raw(Box::new(State::default()))
}

#[no_mangle]
pub extern "C" fn events_handler(event: *const c_char) {
    let cstring = unsafe { CString::from_raw(event.cast_mut()) };
    let value = cstring.to_string_lossy().to_string();
    RUNTIME.spawn(async move {
        let (tx, _) = things::get_pair().await;
        let _ = tx.send(value).await;
    });
    let _ = CString::into_raw(cstring);
}

#[no_mangle]
pub extern "C" fn execute(_state: *mut State, _api: ApiCallbacksMap) {}

#[no_mangle]
pub extern "C" fn handler(_data: *const EventState, _api: ApiCallbacksMap) {}
