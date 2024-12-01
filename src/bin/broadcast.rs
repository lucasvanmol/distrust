use distrust::{init, Body, Message};
use std::io::Write;
use std::{
    collections::{HashMap, HashSet},
    io,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum BroadcastPayload {
    Broadcast {
        message: u64,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<u64>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

fn main() {
    init();

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stderr = io::stderr();
    let mut seen = HashSet::new();
    loop {
        buffer.clear();
        stdin.read_line(&mut buffer).expect("read from stdin");

        let msg: Message<BroadcastPayload> = serde_json::from_str(&buffer).expect("deser");
        writeln!(&mut stderr, "{:?}", msg);

        match msg.body.payload {
            BroadcastPayload::Broadcast { message } => {
                seen.insert(message);
                let reply = Message {
                    src: msg.dest,
                    dest: msg.src,
                    body: Body {
                        msg_id: 1,
                        in_reply_to: Some(msg.body.msg_id),
                        payload: BroadcastPayload::BroadcastOk,
                    },
                };
                println!("{}", serde_json::to_string(&reply).expect("serialization"));
            }
            BroadcastPayload::BroadcastOk => {}
            BroadcastPayload::Read => {
                let reply = Message {
                    src: msg.dest,
                    dest: msg.src,
                    body: Body {
                        msg_id: 1,
                        in_reply_to: Some(msg.body.msg_id),
                        payload: BroadcastPayload::ReadOk {
                            messages: seen.clone().into_iter().collect(),
                        },
                    },
                };
                println!("{}", serde_json::to_string(&reply).expect("serialization"));
            }
            BroadcastPayload::ReadOk { messages } => {}
            BroadcastPayload::Topology { topology } => {
                let reply = Message {
                    src: msg.dest,
                    dest: msg.src,
                    body: Body {
                        msg_id: 1,
                        in_reply_to: Some(msg.body.msg_id),
                        payload: BroadcastPayload::TopologyOk,
                    },
                };
                println!("{}", serde_json::to_string(&reply).expect("serialization"));
            }
            BroadcastPayload::TopologyOk => {}
        }
    }
}

#[cfg(test)]
mod test {
    use crate::BroadcastPayload;

    #[test]
    fn topology() {
        let json = r#"{
            "type": "topology",
            "topology": {
                "n1": ["n2", "n3"],
                "n2": ["n1"],
                "n3": ["n1"]
            }
        }"#;

        let topology: BroadcastPayload = dbg!(serde_json::from_str(json).expect("ser"));

        match topology {
            BroadcastPayload::Topology { topology } => {
                assert_eq!(topology["n1"], vec!["n2", "n3"]);
            }
            _ => panic!("unexpected"),
        }
    }
}
