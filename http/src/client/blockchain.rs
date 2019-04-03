use crate::client::{HttpClient, Result};

use crate::responses;

impl HttpClient {
    pub fn get_block_header(&self, height: u32, cp_height: u32) -> Result<responses::BlockHeader> {
        let uri = format!("{}/nomenclate/block/{}/header", self.uri.clone(), height);

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("cp_height", cp_height)])
            .send()?;

        let block_header = resp.json()?;

        Ok(block_header)
    }

    pub fn get_block_headers(
        &self,
        height: u32,
        cp_height: u32,
        count: u32,
    ) -> Result<responses::BlockHeaders> {
        let uri = format!("{}/nomenclate/block/{}/headers", self.uri.clone(), height);

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("cp_height", cp_height)])
            .query(&[("count", count)])
            .send()?;

        let block_headers = resp.json()?;

        Ok(block_headers)
    }

    pub fn get_estimate_fee(&self, blocks_count: u32) -> Result<responses::Fee> {
        let uri = format!("{}/nomenclate/blockchain/estimatefee", self.uri.clone());

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("blocks_count", blocks_count)])
            .send()?;

        let estimate_fee = resp.json()?;

        Ok(estimate_fee)
    }

    pub fn get_relay_fee(&self) -> Result<responses::Fee> {
        let uri = format!("{}/nomenclate/blockchain/relayfee", self.uri.clone());

        let mut resp = self.client.get(&uri).send()?;

        let relay_fee = resp.json()?;

        Ok(relay_fee)
    }
}
