use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct Meta {
    locale: String,
    timezone: String,
    // interfaces: String
}
