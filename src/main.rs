extern crate mars;
extern crate playpen;

use mars::{Bot, Response};
use std::borrow::Cow;
use std::error::Error;

fn normalize(mut msg: &str) -> &str {
    if msg.starts_with("playbot") {
        msg = &msg[7..];

        if msg.starts_with(':') {
            msg = &msg[1..];
        }
    }

    msg = msg.trim();

    if msg.starts_with("```") {
        msg = &msg[3..];

        if msg.starts_with("rust\n") {
            msg = &msg[4..];
        }
    } else if msg.starts_with('`') {
        msg = &msg[1..];
    }

    msg.trim_right_matches('`').trim()
}

fn process(msg: &str) -> Cow<str> {
    let norm = normalize(msg);

    if norm.contains("fn main") {
        norm.into()
    } else {
        format!("fn main() {{ println!(\"{{:?}}\", {{\n{}\n}}); }}", norm).into()
    }
}

fn main() {
    Bot::new(env!("TOKEN"), |req| {
        let res = playpen::eval(&*process(&req.text));

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

#[cfg(test)]
#[test]
fn test_normalize() {
    assert_eq!(normalize("playbot blah"), "blah");
    assert_eq!(normalize("playbot: blah"), "blah");
    assert_eq!(normalize("playbot: ```\nabc\n```"), "abc");
    assert_eq!(normalize("playbot: ```rust\n\
                         abc\n\
                         ```"), "abc");
    assert_eq!(normalize("playbot: `lulz`"), "lulz");
    assert_eq!(normalize("playbot `lulz`"), "lulz");
    assert_eq!(normalize("playbot :lal"), ":lal");
}

#[cfg(test)]
#[test]
fn test_process() {
    assert_eq!(process("use std::io; fn main() { /* blah */ }"), "use std::io; fn main() { /* blah */ }");
    assert_eq!(process("playbot use std::io; fn main() { /* blah */ }"), "use std::io; fn main() { /* blah */ }");
    assert_eq!(process("playbot: use std::io; fn main() { /* blah */ }"), "use std::io; fn main() { /* blah */ }");
    assert_eq!(process("playbot: `use std::io; fn main() { /* blah */ }`"), "use std::io; fn main() { /* blah */ }");
    assert_eq!(process("playbot: `use std::io;`"), "fn main() { println!(\"{:?}\", {\nuse std::io;\n}); }");
}
