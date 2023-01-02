use serde::{Deserialize, Serialize};

use super::user::User;
use super::application::Application;


#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct VKSession {
    pub session_id: String,
    pub user_id: String,
    pub message_id: i64,
    
    #[serde(skip_serializing)]
    pub skill_id: String,
    #[serde(skip_serializing)]
    pub new: bool,
    #[serde(skip_serializing)]
    pub user: User,
    #[serde(skip_serializing)]
    pub application: Application,
    #[serde(skip_serializing)]
    pub auth_token: String
}

