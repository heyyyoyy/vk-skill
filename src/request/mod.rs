use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Application {
    application_id: String,
    // TODO: create enum types
    application_type: String
}

#[derive(Debug, Deserialize)]
pub struct User {
    user_id: String
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    locale: String,
    timezone: String,
    // interfaces: String
}

#[derive(Debug, Deserialize)]
pub struct Session {
    session_id: String,
    skill_id: String,
    new: bool,
    message_id: i64,
    user: User,
    application: Application,
    // auth_token: String
}

#[derive(Debug, Deserialize)]
pub struct Request {
    command: String,
    original_utterance: String,
    // TODO: use enum of types
    #[serde(rename = "type")]
    type_: String,
    // payload: Payload,
    // nlu: Nlu
}

#[derive(Debug, Deserialize)]
pub struct VKRequest {
    meta: Meta,
    request: Request,
    session: Session,
    version: String
}
