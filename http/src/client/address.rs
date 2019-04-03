use crate::client::{HttpClient, Result};

use crate::responses;

impl HttpClient {
    pub fn get_address_history(
        &self,
        address: &str,
        offset: u32,
        limit: u32,
    ) -> Result<responses::AddressHistory> {
        let uri = format!(
            "{}/nomenclate/address/{}/history",
            self.uri.clone(),
            address
        );

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("offset", offset)])
            .query(&[("limit", limit)])
            .send()?;

        let history = resp.json()?;

        Ok(history)
    }

    pub fn get_balance(&self, address: &str) -> Result<responses::Balance> {
        let uri = format!(
            "{}/nomenclate/address/{}/balance",
            self.uri.clone(),
            address
        );

        let mut resp = self.client.get(&uri).send()?;

        let balance = resp.json()?;

        Ok(balance)
    }

    pub fn get_unspent(
        &self,
        address: &str,
        offset: u32,
        limit: u32,
    ) -> Result<responses::Unspent> {
        let uri = format!(
            "{}/nomenclate/address/{}/unspent",
            self.uri.clone(),
            address
        );

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("offset", offset)])
            .query(&[("limit", limit)])
            .send()?;

        let unspent = resp.json()?;

        Ok(unspent)
    }

    //TODO mempool
}
