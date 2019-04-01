use crate::client::NomenclateHttpClient;

use crate::responses;

impl NomenclateHttpClient {
    pub fn get_block_header(
        &self,
        height: u32,
        cp_height: u32,
    ) -> Result<responses::BlockHeader, reqwest::Error> {
        let uri = format!("{}/nomenclate/block/{}/header", self.uri.clone(), height);

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("cp_height", cp_height)])
            .send()
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json()
    }

    pub fn get_block_headers(
        &self,
        height: u32,
        cp_height: u32,
        count: u32,
    ) -> Result<responses::BlockHeaders, reqwest::Error> {
        let uri = format!("{}/nomenclate/block/{}/headers", self.uri.clone(), height);

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("cp_height", cp_height)])
            .query(&[("count", count)])
            .send()
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json()
    }

    pub fn get_estimate_fee(&self, blocks_count: u32) -> Result<responses::Fee, reqwest::Error> {
        let uri = format!("{}/nomenclate/blockchain/estimatefee", self.uri.clone());

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("blocks_count", blocks_count)])
            .send()
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json()
    }

    pub fn get_relay_fee(&self) -> Result<responses::Fee, reqwest::Error> {
        let uri = format!("{}/nomenclate/blockchain/relayfee", self.uri.clone());

        let mut resp = self.client.get(&uri).send().unwrap();

        resp.json()
    }
}
