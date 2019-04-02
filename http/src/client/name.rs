use crate::client::HttpClient;

use crate::error::Error;
use crate::responses;

impl HttpClient {
    pub fn get_name_history(
        &self,
        name: &str,
        offset: u32,
        limit: u32,
    ) -> Result<responses::NameHistory, Error> {
        let uri = format!("{}/nomenclate/name/{}/history", self.uri.clone(), name);

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
}
