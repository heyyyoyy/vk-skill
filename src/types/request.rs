use serde::Deserialize;

use super::meta::Meta;
use super::session::VKSession;


#[derive(Default, Debug, Deserialize)]
pub struct Request {
    command: String,
    original_utterance: String,
    // TODO: use enum of types
    #[serde(rename = "type")]
    req_type: String,
    // payload: Payload,
    // nlu: Nlu
}


#[derive(Default, Debug, Deserialize)]
pub struct VKRequest {
    pub meta: Meta,
    pub request: Request,
    pub session: VKSession,
    pub version: String
}
