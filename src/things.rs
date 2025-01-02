use tokio::sync::{mpsc, OnceCell};

use tokio::sync::mpsc::Receiver;

use tokio::sync::Mutex;

use tokio::sync::mpsc::Sender;

pub(crate) async fn get_pair() -> &'static (Sender<String>, Mutex<Receiver<String>>) {
    static ONCE: OnceCell<(Sender<String>, Mutex<Receiver<String>>)> = OnceCell::const_new();
    ONCE.get_or_init(|| async {
        let (tx, rx) = mpsc::channel(32);
        (tx, Mutex::const_new(rx))
    })
    .await
}

