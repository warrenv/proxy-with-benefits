use dotenvy::dotenv;
use lazy_static::lazy_static;
use secrecy::Secret;
use std::env as std_env;

pub const DEFAULT_LB_IP_ADDR: &str = "0.0.0.0";
pub const DEFAULT_LB_PORT: &str = "7777";

// internal-to-external name mapping.
pub mod env {
    pub const LB_IP_ADDR_ENV_VAR: &str = "PROXBEN_LB_IP_ADDR";
    pub const LB_PORT_ENV_VAR: &str = "PROXBEN_LB_PORT";
    pub const LB_SECRET_SOME_ENV_VAR: &str = "PROXBEN_SECRET_SOME";
}

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref LB_IP_ADDR: String = set_bind_ip_addr();
    pub static ref LB_PORT: String = set_bind_port();
    pub static ref LB_SECRET_SOME: Secret<String> = set_secret_some();
}

fn set_bind_ip_addr() -> String {
    dotenv().ok();
    std_env::var(env::LB_IP_ADDR_ENV_VAR).unwrap_or(DEFAULT_LB_IP_ADDR.to_owned())
}

fn set_bind_port() -> String {
    dotenv().ok();
    std_env::var(env::LB_PORT_ENV_VAR).unwrap_or(DEFAULT_LB_PORT.to_owned())
}

fn set_secret_some() -> Secret<String> {
    dotenv().ok();

    Secret::new(
        std_env::var(env::LB_SECRET_SOME_ENV_VAR).expect("PROXBEN_SECRET_SOME must be set."),
    )
}

// Hard coded values per environment.
pub mod prod {}

pub mod test {
    pub const LB_IP_ADDR: &str = "127.0.0.1";
    pub const LB_PORT: &str = "0";
}
