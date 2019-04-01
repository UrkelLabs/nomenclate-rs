use crate::client::NomenclateHttpClient;

use crate::responses;

impl NomenclateHttpClient {
    pub fn get_address_history(
        &self,
        address: &str,
    ) -> Result<responses::AddressHistory, reqwest::Error> {
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
