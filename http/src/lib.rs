use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
pub struct NomenclateHttpClient {
    client: Client,
    uri: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SlimTX {
    pub tx_hash: String,
    pub height: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddressHistory {
    pub total: u32,
    pub offset: u32,
    pub limit: u32,
    pub result: Option<Vec<SlimTX>>,
}

impl NomenclateHttpClient {
    pub fn new(uri: &str) -> NomenclateHttpClient {
        NomenclateHttpClient {
            client: Client::new(),
            uri: uri.to_string(),
        }
    }

    //TODO wrap the reqwest error type.
    pub fn banner(&self) -> Result<String, reqwest::Error> {
        let uri = format!("{}/nomenclate/banner", self.uri.clone());

        let mut resp = self.client.get(&uri).send().unwrap();

        resp.text()
    }

    pub fn address_history(&self, address: &str) -> Result<AddressHistory, reqwest::Error> {
        let uri = format!(
            "{}/nomenclate/address/{}/history",
            self.uri.clone(),
            address
        );

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("offset", 0)])
            .query(&[("limit", 25)])
            .send()
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json()
    }
}
