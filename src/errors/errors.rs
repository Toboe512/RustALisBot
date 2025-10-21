use log::error;
use reqwest::Error;

pub fn log_err<T: ToString>(msg: &str, err: T) -> String {
    error!("{}: {}", msg, err.to_string());
    format!("{}: {}", msg, err.to_string())
}

pub fn log_err3<T: ToString>(msg: &str, ttl: &str, err: T) -> String {
    error!("{}: {}: {}", msg, ttl, err.to_string());
    format!("{}: {}: {}", msg, ttl, err.to_string())
}
