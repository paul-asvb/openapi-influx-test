use serde::Deserialize;
use tracing::Level;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Production,
    Development,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    #[serde(default = "default_env")]
    pub env: Environment,

    #[serde(default = "default_port")]
    pub port: u16,

    pub cf_access_key_id: String,
    pub cf_secret_access_key: String,
    pub cf_account_id: String,
    pub cf_bucket_name: String,
}

fn default_port() -> u16 {
    8080
}

fn default_env() -> Environment {
    Environment::Development
}

impl Configuration {
    pub fn socket_addr(&self) -> [u8; 4] {
        match self.env {
            Environment::Production => {
                let socket = [0, 0, 0, 0];
                tracing::info!(
                    "Starting Production on {:?}:{}",
                    socket.as_slice(),
                    self.port
                );
                socket
            }
            Environment::Development => {
                let socket = [127, 0, 0, 1];
                tracing::info!(
                    "Starting Development on {:?}:{}",
                    socket.as_slice(),
                    self.port
                );
                socket
            }
        }
    }

    pub fn log_level(&self) -> Level {
        match self.env {
            Environment::Production => Level::INFO,
            Environment::Development => Level::DEBUG,
        }
    }
}
