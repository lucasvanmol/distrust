use std::io;

use distrust::{Body, Init, InitOk, Message};
use echo::{Echo, EchoOk};
mod echo;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("read from stdin");
    let msg: Message<Init> = serde_json::from_str(&buffer).expect("parse message");

    let id = msg.body.payload.node_id;
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
    loop {
        buffer.clear();
        stdin.read_line(&mut buffer).expect("read from stdin");

        let echo: Message<Echo> = serde_json::from_str(&buffer).expect("deser");
        let reply: Message<EchoOk> = Message {
            src: echo.dest,
            dest: echo.src,
            body: Body {
                msg_id: echo.body.msg_id,
                in_reply_to: Some(echo.body.msg_id),
                payload: EchoOk {
                    echo: echo.body.payload.echo,
                },
            },
        };
        println!("{}", serde_json::to_string(&reply).expect("serialization"));
    }
}
