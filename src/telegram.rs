use std::ffi::{c_char, CString};

use crate::config::{self, Config};
use crate::things;

// use super::AsyaResponse;

use plugin_interface::ApiCallbacksMap;
use teloxide::prelude::Requester;
use teloxide::types::Message;

use teloxide::Bot;

use tokio::sync::OnceCell;

use super::RUNTIME;

pub(crate) fn run_tgbot(api: ApiCallbacksMap, config: Config) {
    // let void_ptr_func: unsafe extern "C" fn(request: *mut c_char) =
    //     unsafe { std::mem::transmute(api.callback("send_human_request")) };

    RUNTIME.spawn(async move {
        config::CONFIG_INSTANCE
            .get_or_init(|| async { config })
            .await;
        static LOOP: OnceCell<()> = OnceCell::const_new();
        let bot = Bot::from_env();
        teloxide::repl(bot, move |bot: Bot, msg: Message| async move {
            let allowed_users = config::CONFIG_INSTANCE.get().unwrap().allowed_users.clone();
            if allowed_users.contains(&msg.chat.username().unwrap().to_string()) {
                LOOP.get_or_init(|| async {
                    RUNTIME.spawn(async move {
                        let (_, mtx) = things::get_pair().await;
                        loop {
                            let mut lock = mtx.lock().await;
                            let res = lock.recv().await.unwrap();
                            if res.contains("AsyaResponse") {
                                let value =
                                    &serde_json::from_str::<serde_json::Value>(&res).unwrap();
                                let _ = bot
                                    .send_message(
                                        msg.chat.id,
                                        serde_json::to_string(
                                            &value.pointer("/eventBody/message").unwrap(),
                                        )
                                        .unwrap(),
                                    )
                                    .await;
                            }
                        }
                    });
                })
                .await;

                let cstring = CString::new(msg.text().unwrap()).unwrap();

                // unsafe { (void_ptr_func)(cstring.into_raw()) };
            }
            Ok(())
        })
        .await;
    });
}
