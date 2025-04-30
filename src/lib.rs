use lazy_static::lazy_static;
use std::ffi::{c_char, CStr, CString};
use tokio::runtime::Runtime;

use plugin_interface::{ApiCallbacksMap, NativePluginInfoCallback, NativePluginInformation};
use serde::Deserialize;

mod config;
mod telegram;
mod things;

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

#[no_mangle]
pub static plugin_info: NativePluginInfoCallback = plugin_information;

#[no_mangle]
pub unsafe extern "C" fn plugin_information() -> *const NativePluginInformation {
    let plugin_name = CString::new("asya_telegram").unwrap();
    let name = plugin_name.into_raw().cast_const();
    let plugin_information = NativePluginInformation {
        init_callback: init,
        name,
    };

    Box::into_raw(Box::new(plugin_information)).cast_const()
}

#[derive(Debug, Deserialize, Clone)]
struct AsyaResponse {
    pub message: String,
}

#[no_mangle]
pub extern "C" fn init(config: *const c_char, api: ApiCallbacksMap) {
    if config.is_null() {
        println!("Config is null");
        return;
    }
    let config = things::extract_config(config);
    unsafe {
        let callback_ptr = api.callback("subscribe_to_events");
        if callback_ptr.is_null() {
            println!("Subscribe to events callback is null");
            return;
        }

        let subscribe_fn: unsafe extern "C" fn(unsafe extern "C" fn(*const c_char)) =
            std::mem::transmute(callback_ptr);

        subscribe_fn(events_handler);
    }

    telegram::run_tgbot(api, config);
}

#[no_mangle]
pub unsafe extern "C" fn events_handler(event: *const c_char) {
    let cstring = unsafe { CStr::from_ptr(event) };
    let value = cstring.to_string_lossy().to_string();
    RUNTIME.spawn(async move {
        let (tx, _) = things::get_pair().await;
        let _ = tx.send(value).await;
    });
}
