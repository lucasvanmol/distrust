use std::io;

use distrust::{Init, InitOk, Message};
use echo::{Echo, EchoOk};
mod echo;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("read from stdin");
    let msg: Message<Init> = serde_json::from_str(&buffer).expect("parse message");

    let id = msg.body.node_id;
    let reply: Message<InitOk> = Message {
        src: msg.dest,
        dest: msg.src,
        body: InitOk {
            in_reply_to: msg.body.msg_id,
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
            body: EchoOk {
                msg_id: echo.body.msg_id,
                in_reply_to: echo.body.msg_id,
                echo: echo.body.echo,
            },
        };
        println!("{}", serde_json::to_string(&reply).expect("serialization"));
    }
}
