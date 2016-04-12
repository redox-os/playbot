extern crate mars;
extern crate playpen;

use mars::{Bot, Response};
use std::error::Error;

fn main() {
    Bot::new(env!("TOKEN"), |req| {
        let res = playpen::eval(req.text.trim_left_matches("playbot:"));

        Response {
            username: Some("playbot"),
            text: match res {
                Ok(playpen::Response { playpen_error: Some(err), .. }) => err,
                Ok(playpen::Response { output: Some(out), .. }) => out,
                Ok(playpen::Response { rustc: err, .. }) => err,
                Err(e) => e.description().into(),
            },
            icon_url: Some("http://www.rustacean.net/img/rustacean-orig-trans.png"),
        }
    }).init("127.0.0.1:80").unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
