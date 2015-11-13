extern crate vkrs;
extern crate serde_json;
extern crate serde;
extern crate hyper;

use std::io::{BufRead, Read, Write};
use std::io::{stdin, stderr};
use std::fs::File;
use hyper::client::{Client, IntoUrl};
use vkrs::api::{WithToken, VkResult};
use vkrs::auth::{OAuthReq, Permission, AccessTokenResult};
use vkrs::audio::{AudioSearchReq, AudioGetResp};

fn get_access_token() -> AccessTokenResult {
    let body = File::open("token.json").and_then(|mut f| {
        let mut buf = String::new();
        f.read_to_string(&mut buf).map(|_| serde_json::from_str(&*buf).and_then(serde_json::value::from_value))
    });

    if let Ok(Ok(body)) = body {
        body
    } else {
        let mut auth_req = OAuthReq::new(env!("VK_APP_ID"));
        auth_req.scope(Permission::Audio);
        println!("Go to {} and enter code below...", auth_req.into_url().unwrap().serialize());

        let inp = stdin();
        let code = {
            let mut buf = String::new();
            inp.read_line(&mut buf).unwrap();
            buf
        };

        let access_token_req = auth_req.to_access_token_req(env!("VK_APP_SECRET"), code.trim());
        let mut buf = String::new();
        Client::new().get(access_token_req.into_url().unwrap()).send().unwrap().read_to_string(&mut buf).unwrap();
        serde_json::from_str(&buf).and_then(serde_json::value::from_value).unwrap()
    }
}

fn main() {
    let token = get_access_token().0.unwrap();
    writeln!(stderr(), "Token: {:?}", token).unwrap();

    let mut buf = String::new();
    let url = AudioSearchReq::new("Poets of the fall").performer_only(true).count(200).with_token(&token).into_url().unwrap();
    println!("{:?}", url);
    Client::new().get(url).send().unwrap().read_to_string(&mut buf).unwrap();
    let result = serde_json::from_str(&buf).and_then(serde_json::value::from_value);
    println!("{:?}", result);
    let songs: VkResult<AudioGetResp> = result.unwrap();

    if let VkResult(Ok(songs)) = songs {
        for song in songs.items {
            println!("{}\t\"{} - {}.mp3\"", song.url, song.artist, song.title);
        }
    }
}
