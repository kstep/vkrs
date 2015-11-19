extern crate vkrs;
extern crate serde_json;
extern crate serde;
extern crate clap;

use std::io::{BufRead, Read, Write};
use std::io::stdin;
use std::fs::File;
use std::env;
use clap::{Arg, App};
use vkrs::*;

static TOKEN_FILE: &'static str = "token.json";

fn fetch_access_token() -> AccessTokenResult {
    let mut auth_req = OAuth::new(env::var("VK_APP_ID").unwrap());
    auth_req.scope(Permission::Audio);
    println!("Go to {} and enter code below...", auth_req.into_url().unwrap().serialize());

    let inp = stdin();
    let code = {
        let mut buf = String::new();
        inp.read_line(&mut buf).unwrap();
        buf
    };

    let access_token_req = auth_req.to_access_token_request(env::var("VK_APP_SECRET").unwrap(), code.trim());
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

fn find_songs(token: &AccessToken, query: &str, performer_only: bool) {
    let url = AudioSearch::new(query).performer_only(performer_only).count(200).with_token(token).into_url().unwrap();

    let mut buf = String::new();
    Client::new().get(url).send().unwrap().read_to_string(&mut buf).unwrap();

    let result = serde_json::from_str(&buf).and_then(serde_json::value::from_value);

    let songs: VkResult<Collection<Audio>> = result.unwrap();

    match songs.0 {
        Ok(songs) => for song in songs.items {
            println!("{}\t\"{} - {}.mp3\"", song.url, song.artist, song.title);
        },
        Err(VkError { error_code: VkErrorCode::Unauthorized, .. }) =>
            find_songs(&fetch_access_token().0.unwrap(), query, performer_only),
        Err(err) => println!("Error: {}", err)
    }
}

fn main() {
    let args = App::new("songs")
        .author("Konstantin Stepanov <me@kstep.me>")
        .version("0.1.0")
        .about("Search for songs in VK")
        .arg(Arg::with_name("query")
             .required(true)
             .help("Query string"))
        .arg(Arg::with_name("performer")
             .short("p")
             .help("Lookup performers only"))
        .get_matches();

    let token = get_access_token().0.unwrap();

    let query = args.value_of("query").unwrap();
    let performer_only = args.is_present("performer");

    find_songs(&token, query, performer_only);
}
