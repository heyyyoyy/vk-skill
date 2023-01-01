use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Application {
    application_id: String,
    // TODO: create enum types
    application_type: String
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct User {
    user_id: String
}

#[derive(Default, Debug, Deserialize)]
pub struct Meta {
    locale: String,
    timezone: String,
    // interfaces: String
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Session {
    session_id: String,
    skill_id: String,
    new: bool,
    message_id: i64,
    user: User,
    application: Application,
    // auth_token: String
}

#[derive(Default, Debug, Deserialize)]
pub struct Request {
    command: String,
    original_utterance: String,
    // TODO: use enum of types
    #[serde(rename = "type")]
    type_: String,
    // payload: Payload,
    // nlu: Nlu
}

#[derive(Default, Debug, Deserialize)]
pub struct VKRequest {
    meta: Meta,
    request: Request,
    session: Session,
    version: String
}

impl VKRequest {
    pub(crate) fn get_session(&self) -> (String, String, i64) {
        (self.session.session_id.clone(), self.session.application.application_id.clone(), self.session.message_id)
    }

    pub(crate) fn get_version(&self) -> String {
        self.version.clone()
    }
}
