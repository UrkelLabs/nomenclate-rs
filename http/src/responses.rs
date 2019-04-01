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
