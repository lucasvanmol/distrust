use distrust::Payload;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(tag = "type")]
pub struct Echo {
    pub msg_id: u64,
    pub echo: String,
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename = "echo_ok")]
pub struct EchoOk {
    pub msg_id: u64,
    pub in_reply_to: u64,
    pub echo: String,
}

impl Payload for Echo {}
impl Payload for EchoOk {}
