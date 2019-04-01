use crate::responses;

use crate::client::NomenclateHttpClient;

impl NomenclateHttpClient {
    pub fn get_banner(&self) -> Result<responses::Banner, reqwest::Error> {
        let uri = format!("{}/nomenclate/banner", self.uri.clone());

        let mut resp = self.client.get(&uri).send().unwrap();

        resp.json()
    }

    pub fn get_features(&self) -> Result<responses::Features, reqwest::Error> {
        let uri = format!("{}/nomenclate/features", self.uri.clone());

        let mut resp = self.client.get(&uri).send().unwrap();

        resp.json()
    }

    pub fn ping(&self) -> Result<(), reqwest::Error> {
        let uri = format!("{}/nomenclate/ping", self.uri.clone());

        let _resp = self.client.get(&uri).send().unwrap();

        Ok(())
    }

    pub fn get_version(&self) -> Result<responses::Version, reqwest::Error> {
        let uri = format!("{}/nomenclate/version", self.uri.clone());

        let mut resp = self.client.get(&uri).send().unwrap();

        resp.json()
    }
}
