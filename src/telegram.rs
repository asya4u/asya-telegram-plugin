use std::ffi::CString;

use crate::things;

use super::AsyaResponse;

use teloxide::prelude::Requester;
use teloxide::types::Message;

use teloxide::Bot;

use tokio::sync::OnceCell;

use super::RUNTIME;

use plugin_interface::ApiCallbacks;

pub(crate) fn run_tgbot(api: ApiCallbacks) {
    RUNTIME.spawn(async move {
        static LOOP: OnceCell<()> = OnceCell::const_new();
        let bot = Bot::from_env();
        teloxide::repl(bot, move |bot: Bot, msg: Message| async move {
            LOOP.get_or_init(|| async {
                RUNTIME.spawn(async move {
                    let (_, mtx) = things::get_pair().await;
                    loop {
                        let mut lock = mtx.lock().await;
                        let res = lock.recv().await.unwrap();
                        if res.contains("asyaResponse") {
                            let value = &serde_json::from_str::<AsyaResponse>(&res).unwrap();
                            let _ = bot.send_message(msg.chat.id, value.message.clone()).await;
                        }
                    }
                });
            })
            .await;
            let cstring = CString::new(msg.text().unwrap()).unwrap();
            unsafe {
                (api.send_human_request)(cstring.into_raw());
            }
            Ok(())
        })
        .await;
    });
}
