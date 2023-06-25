 use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Field {
    pub field: [[u8; 9]; 9],
}
