extern crate vkrs;
extern crate serde_json;
extern crate serde;
extern crate hyper;

use std::io::{BufRead, Read};
use std::io::stdin;
use hyper::client::Client;
use vkrs::auth::{OAuthReq, Permission, AccessTokenResp};

fn main() {
    let mut auth_req = OAuthReq::new(env!("VK_APP_ID"));
    auth_req.scope(Permission::Audio);
    println!("Go to {} and enter code below...", auth_req.to_url().serialize());

    let inp = stdin();
    let code = {
        let mut buf = String::new();
        inp.read_line(&mut buf).unwrap();
        buf
    };

    let access_token_req = auth_req.to_access_token_req(env!("VK_APP_SECRET"), code.trim());
    let mut response = Client::new().get(access_token_req.to_url()).send().unwrap();
    let body: AccessTokenResp = {
        let mut buf = String::new();
        response.read_to_string(&mut buf).unwrap();
        serde_json::from_str(&buf).and_then(serde_json::value::from_value).unwrap()
    };

    println!("{:?}", body);
}
