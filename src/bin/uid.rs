use rand::prelude::*;
use std::io;

use distrust::{init, Body, Message};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum UidPayload {
    Generate,
    GenerateOk {
        #[serde(rename = "id")]
        guid: u64,
    },
}

fn main() {
    init();

    let mut rng = rand::thread_rng();
    let mut buffer = String::new();
    let stdin = io::stdin();
    loop {
        buffer.clear();
        stdin.read_line(&mut buffer).expect("read from stdin");

        let msg: Message<UidPayload> = serde_json::from_str(&buffer).expect("deser");
        let reply = Message {
            src: msg.dest,
            dest: msg.src,
            body: Body {
                msg_id: msg.body.msg_id,
                in_reply_to: Some(msg.body.msg_id),
                // rng.gen() is good enough for this test
                // otherwise could use combination with node id, keep a counter etc.
                payload: UidPayload::GenerateOk { guid: rng.gen() },
            },
        };
        println!("{}", serde_json::to_string(&reply).expect("serialization"));
    }
}
