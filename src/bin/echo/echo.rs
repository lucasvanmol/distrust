use distrust::Payload;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "type")]
pub struct Echo {
    pub echo: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename = "echo_ok")]
pub struct EchoOk {
    pub echo: String,
}

impl Payload for Echo {}
impl Payload for EchoOk {}
