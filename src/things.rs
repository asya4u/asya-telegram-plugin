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
            return Config::default();
        }
        let cstr = CStr::from_ptr(config);
        match cstr.to_str() {
            Err(_) => {
                eprintln!("Error: failed to convert C string to Rust string");
                return Config::default();
            }
            Ok(casted_str) => {
                println!("bebra blya {}", casted_str);
                match serde_json::from_str::<config::Config>(casted_str) {
                    Err(_) => {
                        eprintln!("Error: failed to parse JSON string");
                        Config::default()
                    }
                    Ok(config) => config,
                }
            }
        }
    }
}
