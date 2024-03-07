use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub body: String,
}