use std::io::{self, Stdin};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message<P: Payload> {
    pub src: String,
    pub dest: String,
    pub body: Body<P>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body<P: Payload> {
    pub msg_id: u64,
    pub in_reply_to: Option<u64>,
    #[serde(flatten)]
    pub payload: P,
}

pub trait Payload {}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename = "init_ok")]
pub struct InitOk {}

impl Payload for Init {}
impl Payload for InitOk {}

pub fn init() -> Init {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("read from stdin");
    let msg: Message<Init> = serde_json::from_str(&buffer).expect("parse message");

    let reply: Message<InitOk> = Message {
        src: msg.dest,
        dest: msg.src,
        body: Body {
            msg_id: 1,
            in_reply_to: Some(msg.body.msg_id),
            payload: InitOk {},
        },
    };

    println!("{}", serde_json::to_string(&reply).expect("serialization"));

    return msg.body.payload;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize() {
        let reply_ok: Message<InitOk> = Message {
            src: "n1".to_owned(),
            dest: "c1".to_owned(),
            body: Body {
                msg_id: 1,
                in_reply_to: Some(1),
                payload: InitOk {},
            },
        };

        let ser = serde_json::to_string(&reply_ok).unwrap();

        println!("{}", ser);
    }

    #[test]
    fn deserialize() {
        let buffer = r#"
        {
            "src": "c0",
            "dest": "n3",
            "body": {
                "type":     "init",
                "msg_id":   1,
                "node_id":  "n3",
                "node_ids": ["n1", "n2", "n3"]
            }
        }   
        "#
        .to_owned();
        let msg: Message<Init> = serde_json::from_str(&buffer).expect("parse message");
        dbg!(msg);
    }
}
