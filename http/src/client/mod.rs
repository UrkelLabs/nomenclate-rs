use crate::error::Error;
use reqwest::Client;
use std::result;

mod address;
mod blockchain;
mod name;
mod server;
mod transaction;

pub struct HttpClient {
    client: Client,
    uri: String,
}

pub type Result<T> = result::Result<T, Error>;

impl HttpClient {
    pub fn new(uri: &str) -> HttpClient {
        HttpClient {
            client: Client::new(),
            uri: uri.to_string(),
        }
    }
}
