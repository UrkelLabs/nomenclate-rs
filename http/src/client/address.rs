use crate::client::NomenclateHttpClient;

use crate::responses;

impl NomenclateHttpClient {
    pub fn get_address_history(
        &self,
        address: &str,
        offset: u32,
        limit: u32,
    ) -> Result<responses::AddressHistory, reqwest::Error> {
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
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json()
    }

    pub fn get_balance(&self, address: &str) -> Result<responses::Balance, reqwest::Error> {
        let uri = format!(
            "{}/nomenclate/address/{}/balance",
            self.uri.clone(),
            address
        );

        let mut resp = self.client.get(&uri).send().unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json()
    }

    pub fn get_unspent(
        &self,
        address: &str,
        offset: u32,
        limit: u32,
    ) -> Result<responses::Unspent, reqwest::Error> {
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
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.
        resp.json()
    }

    //TODO mempool
}
