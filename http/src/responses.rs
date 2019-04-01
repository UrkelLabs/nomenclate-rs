use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SlimTX {
    pub tx_hash: String,
    pub height: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddressHistory {
    pub total: u32,
    pub offset: u32,
    pub limit: u32,
    pub result: Option<Vec<SlimTX>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlockHeader {
    pub branch: Option<Vec<String>>,
    pub header: String,
    pub root: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlockHeaders {
    pub count: u32,
    pub max: u32,
    pub branch: Option<Vec<String>>,
    pub hex: String,
    pub root: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Fee {
    pub fee: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Balance {
    pub confirmed: u64,
    pub unconfirmed: u64,
    pub received: u64,
    pub spent: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Utxo {
    pub tx_hash: String,
    pub height: u32,
    pub tx_pos: u32,
    pub value: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Unspent {
    pub total: u32,
    pub offset: u32,
    pub limit: u32,
    pub result: Option<Vec<Utxo>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionHex {
    pub hex: String,
    pub merkle: Option<Vec<String>>,
    pub block_height: Option<u32>,
    pub pos: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionMerkle {
    pub merkle: Vec<String>,
    pub block_height: u32,
    pub pos: u32,
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct TransactionVerbose {
//     pub txid: String,
//     pub hash: String,
//     pub size: u32,
//     pub vsize: u32,
//     pub version: u32,
//     pub locktime: u32,
// }

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TransactionHash {
    pub tx_hash: String,
    pub merkle: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BroadcastedTx {
    pub hash: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Banner {
    pub banner: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Features {
    pub genesis_hash: String,
    pub hosts: HashMap<String, HostConfig>,
    pub protocol_max: String,
    pub protocol_min: String,
    pub server_version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostConfig {
    pub port: u32,
    pub ssl: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Version {
    pub version: String,
    pub protocol: Vec<String>,
}
