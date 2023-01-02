use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub user_id: String
}
