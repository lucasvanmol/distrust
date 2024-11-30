use rand::prelude::*;
use std::io;

use distrust::{init, Body, Message, Payload};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Generate {}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename = "generate_ok")]
pub struct GenerateOk {
    id: u64,
}

impl Payload for Generate {}
impl Payload for GenerateOk {}

fn main() {
    init();

    let mut rng = rand::thread_rng();
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        buffer.clear();
        stdin.read_line(&mut buffer).expect("read from stdin");

        let echo: Message<Generate> = serde_json::from_str(&buffer).expect("deser");
        let reply: Message<GenerateOk> = Message {
            src: echo.dest,
            dest: echo.src,
            body: Body {
                msg_id: echo.body.msg_id,
                in_reply_to: Some(echo.body.msg_id),
                payload: GenerateOk { id: rng.gen() },
            },
        };
        println!("{}", serde_json::to_string(&reply).expect("serialization"));
    }
}
