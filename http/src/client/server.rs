use crate::responses;

use crate::client::HttpClient;
use crate::error::Error;

impl HttpClient {
    pub fn get_banner(&self) -> Result<responses::Banner, Error> {
        let uri = format!("{}/nomenclate/banner", self.uri.clone());

        let mut resp = self.client.get(&uri).send().map_err(Error::from).unwrap();

        resp.json().map_err(Error::from)
    }

    pub fn get_features(&self) -> Result<responses::Features, Error> {
        let uri = format!("{}/nomenclate/features", self.uri.clone());

        let mut resp = self.client.get(&uri).send().map_err(Error::from).unwrap();

        resp.json().map_err(Error::from)
    }

    pub fn ping(&self) -> Result<(), Error> {
        let uri = format!("{}/nomenclate/ping", self.uri.clone());

        let _resp = self.client.get(&uri).send().map_err(Error::from).unwrap();

        Ok(())
    }

    pub fn get_version(&self) -> Result<responses::Version, Error> {
        let uri = format!("{}/nomenclate/version", self.uri.clone());

        let mut resp = self.client.get(&uri).send().map_err(Error::from).unwrap();

        resp.json().map_err(Error::from)
    }
}
