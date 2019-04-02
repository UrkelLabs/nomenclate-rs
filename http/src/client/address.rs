use crate::client::HttpClient;

use crate::error::Error;
use crate::responses;

impl HttpClient {
    pub fn get_address_history(
        &self,
        address: &str,
        offset: u32,
        limit: u32,
    ) -> Result<responses::AddressHistory, Error> {
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
            .send()
            .map_err(Error::from)
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json().map_err(Error::from)
    }

    pub fn get_balance(&self, address: &str) -> Result<responses::Balance, Error> {
        let uri = format!(
            "{}/nomenclate/address/{}/balance",
            self.uri.clone(),
            address
        );

        let mut resp = self.client.get(&uri).send().map_err(Error::from).unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json().map_err(Error::from)
    }

    pub fn get_unspent(
        &self,
        address: &str,
        offset: u32,
        limit: u32,
    ) -> Result<responses::Unspent, Error> {
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
            .send()
            .map_err(Error::from)
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.
        resp.json().map_err(Error::from)
    }

    //TODO mempool
}
