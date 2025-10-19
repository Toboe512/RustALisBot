use reqwest::Error;
use log::error;


pub fn log_err<T: ToString>(msg: &str, err: T) -> String {
    error!("{}: {}", msg, err.to_string());
    format!("{}: {}", msg, err.to_string())
}