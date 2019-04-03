use crate::client::{HttpClient, Result};

use crate::responses;

impl HttpClient {
    pub fn get_banner(&self) -> Result<responses::Banner> {
        let uri = format!("{}/nomenclate/banner", self.uri.clone());

        let mut resp = self.client.get(&uri).send()?;

        let banner = resp.json()?;

        Ok(banner)
    }

    pub fn get_features(&self) -> Result<responses::Features> {
        let uri = format!("{}/nomenclate/features", self.uri.clone());

        let mut resp = self.client.get(&uri).send()?;

        let features = resp.json()?;

        Ok(features)
    }

    pub fn ping(&self) -> Result<()> {
        let uri = format!("{}/nomenclate/ping", self.uri.clone());

        let _resp = self.client.get(&uri).send()?;

        Ok(())
    }

    pub fn get_version(&self) -> Result<responses::Version> {
        let uri = format!("{}/nomenclate/version", self.uri.clone());

        let mut resp = self.client.get(&uri).send()?;

        let version = resp.json()?;

        Ok(version)
    }
}
