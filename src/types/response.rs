use serde::Serialize;

use super::{request::VKRequest, session::VKSession};


// mock structs
#[derive(Default, Debug, Serialize)]
pub struct Button;

#[derive(Default, Debug, Serialize)]
pub struct Card;


#[derive(Default, Debug, Serialize)]
pub struct Response {
    // string or array
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<Button>>,
    pub end_session: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commands: Option<Vec<String>>
}


#[derive(Default, Debug, Serialize)]
pub struct VKResponse {
    pub response: Response,
    pub session: VKSession,
    pub version: String
}


impl VKResponse {
    pub async fn from_request(req: VKRequest) -> Self {
        let mut session = req.session;
        session.user_id = session.application.application_id.clone();
        Self {
            response: Response::default(),
            session,
            version: req.version
        }
    }
}

