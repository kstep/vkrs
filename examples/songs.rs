extern crate vkrs;
extern crate serde_json;
extern crate serde;
extern crate hyper;

use std::io::{BufRead, Read, Write};
use std::io::stdin;
use std::fs::File;
use hyper::client::{Client, IntoUrl};
use vkrs::api::{WithToken, VkResult, VkError, VkErrorCode};
use vkrs::auth::{OAuthReq, Permission, AccessTokenResult, AccessTokenResp};
use vkrs::audio::{AudioSearchReq, AudioGetResp};

static TOKEN_FILE: &'static str = "token.json";

fn fetch_access_token() -> AccessTokenResult {
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
    let _ = File::create(TOKEN_FILE).and_then(|mut f| f.write_all(buf.as_bytes()));
    serde_json::from_str(&buf).and_then(serde_json::value::from_value).unwrap()
}

fn get_access_token() -> AccessTokenResult {
    let body = File::open(TOKEN_FILE).and_then(|mut f| {
        let mut buf = String::new();
        f.read_to_string(&mut buf).map(|_| serde_json::from_str(&*buf).and_then(serde_json::value::from_value))
    });

    if let Ok(Ok(body)) = body {
        body
    } else {
        fetch_access_token()
    }
}

fn find_songs_by_performer(token: &AccessTokenResp, performer: &'static str) {
    let url = AudioSearchReq::new(performer).performer_only(true).count(200).with_token(token).into_url().unwrap();

    let mut buf = String::new();
    Client::new().get(url).send().unwrap().read_to_string(&mut buf).unwrap();

    let result = serde_json::from_str(&buf).and_then(serde_json::value::from_value);

    let songs: VkResult<AudioGetResp> = result.unwrap();

    match songs.0 {
        Ok(songs) => for song in songs.items {
            println!("{}\t\"{} - {}.mp3\"", song.url, song.artist, song.title);
        },
        Err(VkError { error_code: VkErrorCode::Unauthorized, .. }) =>
            find_songs_by_performer(&fetch_access_token().0.unwrap(), performer),
        Err(err) => println!("Error: {}", err)
    }
}

fn main() {
    let token = get_access_token().0.unwrap();
    find_songs_by_performer(&token, "Poets of the fall");
}
