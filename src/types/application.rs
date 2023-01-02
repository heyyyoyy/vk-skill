use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Application {
    pub application_id: String,
    // TODO: create enum types
    pub application_type: String
}
