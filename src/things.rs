use std::ffi::{c_char, CStr};

use tokio::sync::{mpsc, OnceCell};

use tokio::sync::mpsc::Receiver;

use tokio::sync::Mutex;

use tokio::sync::mpsc::Sender;

use crate::config::{self, Config};

pub(crate) async fn get_pair() -> &'static (Sender<String>, Mutex<Receiver<String>>) {
    static ONCE: OnceCell<(Sender<String>, Mutex<Receiver<String>>)> = OnceCell::const_new();
    ONCE.get_or_init(|| async {
        let (tx, rx) = mpsc::channel(32);
        (tx, Mutex::const_new(rx))
    })
    .await
}

pub(crate) fn extract_config(config: *const c_char) -> config::Config {
    unsafe {
        if config.is_null() {
            Config::default()
        } else {
            let cstr = CStr::from_ptr(config);
            let casted_str = cstr.to_str().unwrap_or_default();
            let from_str = serde_json::from_str::<config::Config>(casted_str);
            from_str.unwrap_or_default()
        }
    }
}
