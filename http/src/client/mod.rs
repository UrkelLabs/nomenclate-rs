use reqwest::Client;

mod blockchain;
mod address;
mod server;

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
