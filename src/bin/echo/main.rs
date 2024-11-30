use std::io;

use distrust::{init, Body, Message};
use echo::{Echo, EchoOk};
mod echo;

fn main() {
    init();

    let mut buffer = String::new();
    let stdin = io::stdin();
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
