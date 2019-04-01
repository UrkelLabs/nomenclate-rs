use crate::client::NomenclateHttpClient;

impl NomenclateHttpClient {

    pub fn get_banner(&self) -> Result<String, reqwest::Error> {
        let uri = format!("{}/nomenclate/banner", self.uri.clone());

        let mut resp = self.client.get(&uri).send().unwrap();

        resp.text()
    }

}
