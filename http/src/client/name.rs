use crate::client::{HttpClient, Result};

use crate::responses;

impl HttpClient {
    pub fn get_name_history(
        &self,
        name: &str,
        offset: u32,
        limit: u32,
    ) -> Result<responses::NameHistory> {
        let uri = format!("{}/nomenclate/name/{}/history", self.uri.clone(), name);

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("offset", offset)])
            .query(&[("limit", limit)])
            .send()?;

        let name_history = resp.json()?;

        Ok(name_history)
    }
}
