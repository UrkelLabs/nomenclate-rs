use std::collections::HashMap;

use crate::client::NomenclateHttpClient;

use crate::responses;

impl NomenclateHttpClient {
    pub fn get_transaction_hex(
        &self,
        tx_hash: &str,
        merkle: bool,
    ) -> Result<responses::TransactionHex, reqwest::Error> {
        let uri = format!("{}/nomenclate/transaction/{}", self.uri.clone(), tx_hash);

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("merkle", merkle)])
            .query(&[("verbose", false)])
            .send()
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json()
    }

    //TODO need Handshake Rust client types before I do this, as it'll just be too much repeat.
    //pub fn get_transaction_verbose(&self, tx_hash: String, merkle: bool) -> Result<responses::TransactionVerbose, reqwest::Error> {
    //    let uri = format!("{}/nomenclate/transaction/{}", self.uri.clone(), tx_hash);

    //    let mut resp = self.client.get(&uri)
    //        .query(&[("merkle", merkle)])
    //        .query(&[("verbose", true)])
    //        .send().unwrap();

    //    //Before this, we have to check if the response was a success or not, and return error if
    //    //not.

    //    resp.json()
    //}
    //

    pub fn get_transaction_merkle(
        &self,
        tx_hash: &str,
        block_height: u32,
    ) -> Result<responses::TransactionMerkle, reqwest::Error> {
        let uri = format!(
            "{}/nomenclate/transaction/{}/merkle/{}",
            self.uri.clone(),
            tx_hash,
            block_height
        );

        let mut resp = self.client.get(&uri).send().unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json()
    }

    pub fn get_transaction_by_position(
        &self,
        block_height: u32,
        position: u32,
        merkle: bool,
    ) -> Result<responses::TransactionHash, reqwest::Error> {
        let uri = format!(
            "{}/nomenclate/transaction/{}/byPosition/{}",
            self.uri.clone(),
            block_height,
            position
        );

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("merkle", merkle)])
            .send()
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json()
    }

    pub fn broadcast_transaction(
        &self,
        transaction: &str,
    ) -> Result<responses::BroadcastedTx, reqwest::Error> {
        let uri = format!("{}/nomenclate/transaction/broadcast", self.uri.clone(),);

        let mut map = HashMap::new();

        map.insert("tx", transaction);

        let mut resp = self.client.post(&uri).json(&map).send().unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json()
    }
}
