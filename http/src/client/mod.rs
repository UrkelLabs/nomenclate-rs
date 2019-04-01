use reqwest::Client;

mod address;
mod blockchain;
mod name;
mod server;
mod transaction;

pub struct NomenclateHttpClient {
    client: Client,
    uri: String,
}

impl NomenclateHttpClient {
    pub fn new(uri: &str) -> NomenclateHttpClient {
        NomenclateHttpClient {
            client: Client::new(),
            uri: uri.to_string(),
        }
    }
}
