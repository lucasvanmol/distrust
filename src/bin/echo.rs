use std::io;

use distrust::{init, Body, Message};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum EchoPayload {
    Echo { echo: String },
    EchoOk { echo: String },
}

fn main() {
    init();

    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        buffer.clear();
        stdin.read_line(&mut buffer).expect("read from stdin");

        let msg: Message<EchoPayload> = serde_json::from_str(&buffer).expect("deser");

        match msg.body.payload {
            EchoPayload::Echo { echo } => {
                let reply = Message {
                    src: msg.dest,
                    dest: msg.src,
                    body: Body {
                        msg_id: msg.body.msg_id,
                        in_reply_to: Some(msg.body.msg_id),
                        payload: EchoPayload::EchoOk { echo },
                    },
                };
                println!("{}", serde_json::to_string(&reply).expect("serialization"));
            }
            EchoPayload::EchoOk { .. } => {}
        }
    }
}
