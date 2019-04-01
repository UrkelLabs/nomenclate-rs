use crate::client::NomenclateHttpClient;

use crate::responses;

impl NomenclateHttpClient {
    pub fn get_name_history(
        &self,
        name: &str,
        offset: u32,
        limit: u32,
    ) -> Result<responses::NameHistory, reqwest::Error> {
        let uri = format!("{}/nomenclate/name/{}/history", self.uri.clone(), name);

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
}
