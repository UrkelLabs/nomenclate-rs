use crate::client::HttpClient;

use crate::error::Error;
use crate::responses;

impl HttpClient {
    pub fn get_block_header(
        &self,
        height: u32,
        cp_height: u32,
    ) -> Result<responses::BlockHeader, Error> {
        let uri = format!("{}/nomenclate/block/{}/header", self.uri.clone(), height);

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("cp_height", cp_height)])
            .send()
            .map_err(Error::from)
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json().map_err(Error::from)
    }

    pub fn get_block_headers(
        &self,
        height: u32,
        cp_height: u32,
        count: u32,
    ) -> Result<responses::BlockHeaders, Error> {
        let uri = format!("{}/nomenclate/block/{}/headers", self.uri.clone(), height);

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("cp_height", cp_height)])
            .query(&[("count", count)])
            .send()
            .map_err(Error::from)
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json().map_err(Error::from)
    }

    pub fn get_estimate_fee(&self, blocks_count: u32) -> Result<responses::Fee, Error> {
        let uri = format!("{}/nomenclate/blockchain/estimatefee", self.uri.clone());

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("blocks_count", blocks_count)])
            .send()
            .map_err(Error::from)
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json().map_err(Error::from)
    }

    pub fn get_relay_fee(&self) -> Result<responses::Fee, Error> {
        let uri = format!("{}/nomenclate/blockchain/relayfee", self.uri.clone());

        let mut resp = self.client.get(&uri).send().map_err(Error::from).unwrap();

        resp.json().map_err(Error::from)
    }
}
