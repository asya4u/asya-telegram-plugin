use serde::Deserialize;
use tokio::sync::OnceCell;

pub static CONFIG_INSTANCE: OnceCell<Config> = OnceCell::const_new();

#[derive(Default, Clone, Deserialize)]
pub struct Config {
    // pub telegram_token: Option<String>,
    pub allowed_users: Vec<String>,
}
