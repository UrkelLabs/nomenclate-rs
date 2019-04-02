use std::collections::HashMap;

use crate::client::HttpClient;

use crate::error::Error;
use crate::responses;

impl HttpClient {
    pub fn get_transaction_hex(
        &self,
        tx_hash: &str,
        merkle: bool,
    ) -> Result<responses::TransactionHex, Error> {
        let uri = format!("{}/nomenclate/transaction/{}", self.uri.clone(), tx_hash);

        let mut resp = self
            .client
            .get(&uri)
            .query(&[("merkle", merkle)])
            .query(&[("verbose", false)])
            .send()
            .map_err(Error::from)
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json().map_err(Error::from)
    }

    //TODO need Handshake Rust client types before I do this, as it'll just be too much repeat.
    //pub fn get_transaction_verbose(&self, tx_hash: String, merkle: bool) -> Result<responses::TransactionVerbose, Error> {
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
    ) -> Result<responses::TransactionMerkle, Error> {
        let uri = format!(
            "{}/nomenclate/transaction/{}/merkle/{}",
            self.uri.clone(),
            tx_hash,
            block_height
        );

        let mut resp = self.client.get(&uri).send().map_err(Error::from).unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json().map_err(Error::from)
    }

    pub fn get_transaction_by_position(
        &self,
        block_height: u32,
        position: u32,
        merkle: bool,
    ) -> Result<responses::TransactionHash, Error> {
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
            .map_err(Error::from)
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json().map_err(Error::from)
    }

    pub fn broadcast_transaction(
        &self,
        transaction: &str,
    ) -> Result<responses::BroadcastedTx, Error> {
        let uri = format!("{}/nomenclate/transaction/broadcast", self.uri.clone(),);

        let mut map = HashMap::new();

        map.insert("tx", transaction);

        let mut resp = self
            .client
            .post(&uri)
            .json(&map)
            .send()
            .map_err(Error::from)
            .unwrap();

        //Before this, we have to check if the response was a success or not, and return error if
        //not.

        resp.json().map_err(Error::from)
    }
}
