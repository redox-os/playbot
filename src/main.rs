extern crate mars;
extern crate playpen;

use mars::{Bot, Response};
use std::error::Error;

fn process(msg: &str) -> String {
    String::from("fn main() { println!(\"{:?}\", {\n")
        + msg.trim_left_matches("playbot:")
             .trim()
             .trim_left_matches("```rust")
             .trim_matches('`')
        + "\n}); }"
}

fn main() {
    Bot::new(env!("TOKEN"), |req| {
        let res = playpen::eval(process(&req.text).as_str());

        Response {
            username: Some("playbot".into()),
            text: match res {
                Ok(playpen::Response { playpen_error: Some(err), .. }) => format!(":fire: {}", err),
                Ok(playpen::Response { output: Some(out), .. }) => format!(":cake:\n```\n{}\n```", out),
                Ok(playpen::Response { rustc: err, .. }) => format!(":bomb:\n```\n{}\n```", err),
                Err(e) => format!(":question: {}", e.description()),
            }.into(),
            icon_url: Some("http://www.rustacean.net/assets/rustacean-orig-trans.png".into()),
        }
    }).init(env!("IP")).unwrap();
}
