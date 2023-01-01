use serde::Serialize;
use crate::request::VKRequest;

// mock structs
#[derive(Default, Debug, Serialize)]
pub struct Button;

#[derive(Default, Debug, Serialize)]
pub struct Card;


#[derive(Default, Debug, Serialize)]
pub struct Response {
    // string or array
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    buttons: Option<Vec<Button>>,
    end_session: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    card: Option<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    commands: Option<Vec<String>>
}

#[derive(Default, Debug, Serialize, Clone)]
pub struct ResponseSession {
    session_id: String,
    user_id: String,
    message_id: i64,
}


#[derive(Default, Debug, Serialize)]
pub struct VKResponse {
    response: Response,
    session: ResponseSession,
    version: String
}


impl VKResponse {
    // TODO: Copy session?
    pub fn set_session(&mut self, session: ResponseSession) -> &mut Self {
        self.session = session;
        self
    }

    pub fn create_resp(&mut self, req: &VKRequest) -> &mut Self {
        // set session
        let (session_id, user_id, message_id) = req.get_session();
        
        self.session = ResponseSession {session_id, user_id, message_id};

        // set response
        let mut resp = Response::default();
        // Minimum working example 
        resp.text = "12345".to_string();
        self.response = resp;

        // set version
        self.version = req.get_version();
        
        self
    }
}

