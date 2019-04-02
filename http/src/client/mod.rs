use reqwest::Client;

mod address;
mod blockchain;
mod name;
mod server;
mod transaction;

pub struct HttpClient {
    client: Client,
    uri: String,
}

impl HttpClient {
    pub fn new(uri: &str) -> HttpClient {
        HttpClient {
            client: Client::new(),
            uri: uri.to_string(),
        }
    }
}
