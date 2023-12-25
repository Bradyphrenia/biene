use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseTable {
    pub msg: String,
    pub data: String,
}
