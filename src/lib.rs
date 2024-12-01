use std::io::{self, Stdin};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message<Payload> {
    pub src: String,
    pub dest: String,
    pub body: Body<Payload>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Body<Payload> {
    pub msg_id: u64,
    pub in_reply_to: Option<u64>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum InitPayload {
    Init(Init),
    InitOk,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

pub fn init() -> Init {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("read from stdin");
    let msg: Message<InitPayload> = serde_json::from_str(&buffer).expect("parse message");

    let reply: Message<InitPayload> = Message {
        src: msg.dest,
        dest: msg.src,
        body: Body {
            msg_id: 1,
            in_reply_to: Some(msg.body.msg_id),
            payload: InitPayload::InitOk,
        },
    };

    println!("{}", serde_json::to_string(&reply).expect("serialization"));

    match msg.body.payload {
        InitPayload::Init(init) => init,
        InitPayload::InitOk => todo!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize() {
        let reply_ok: Message<InitPayload> = Message {
            src: "n1".to_owned(),
            dest: "c1".to_owned(),
            body: Body {
                msg_id: 1,
                in_reply_to: Some(1),
                payload: InitPayload::InitOk,
            },
        };

        let ser = serde_json::to_string(&reply_ok).unwrap();

        assert_eq!(
            ser,
            r#"{"src":"n1","dest":"c1","body":{"msg_id":1,"in_reply_to":1,"type":"init_ok"}}"#
        );
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
        let msg: Message<Init> = dbg!(serde_json::from_str(&buffer).expect("parse message"));
        assert_eq!(msg.body.payload.node_ids, vec!["n1", "n2", "n3"]);
    }
}
