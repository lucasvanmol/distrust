use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message<P: Payload> {
    pub src: String,
    pub dest: String,
    pub body: P,
}

pub trait Payload {}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub struct Init {
    pub msg_id: u64,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Serialize)]
#[serde(tag = "type")]
#[serde(rename = "init_ok")]
pub struct InitOk {
    pub in_reply_to: u64,
}

impl Payload for Init {}
impl Payload for InitOk {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize() {
        let reply_ok: Message<InitOk> = Message {
            src: "n1".to_owned(),
            dest: "c1".to_owned(),
            body: InitOk { in_reply_to: 1 },
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
