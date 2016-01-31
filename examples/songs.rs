extern crate vkrs;
extern crate clap;
extern crate serde_json as json;

use std::io::BufRead;
use std::io::stdin;
use std::fs::File;
use std::env;
use clap::{Arg, App};
use vkrs::*;

static TOKEN_FILE: &'static str = "token.json";

fn fetch_access_token() -> Result<AccessToken, OAuthError> {
    let oauth = Client::auth(
        env::var("VK_APP_ID").unwrap(),
        env::var("VK_APP_SECRET").unwrap());

    let auth_uri = oauth.auth_uri(Some(Permission::Audio.as_ref()), None).unwrap();
    println!("Go to {} and enter code below...", auth_uri);

    let inp = stdin();
    let code = {
        let mut buf = String::new();
        inp.read_line(&mut buf).unwrap();
        buf
    };

    let token = try!(oauth.request_token(code.trim()));
    let _ = File::create(TOKEN_FILE).ok().map(|mut f| json::to_writer(&mut f, &token).ok());
    Ok(token)
}

fn get_access_token() -> Result<AccessToken, OAuthError> {
    let token: Option<AccessToken> = File::open(TOKEN_FILE).ok().and_then(|mut f| json::from_reader(&mut f).ok());

    if let Some(token) = token {
        if token.expired() {
            fetch_access_token()
        } else {
            Ok(token)
        }
    } else {
        fetch_access_token()
    }
}

fn print_m3u(songs: &Collection<Audio>) {
    println!("#EXTM3U");
    for song in &songs.items {
        println!("#EXTINF:{},{} - {} ({}_{})", song.duration, song.artist, song.title, song.owner_id, song.id);
        println!("{}", song.url);
    }
}

fn find_songs(token: &AccessToken, query: &str, performer_only: bool) {
    let songs: VkResult<Collection<Audio>> = Client::new().token(token).get(AudioSearch::new(query).performer_only(performer_only).count(200));
    //let songs: VkResult<Collection<Audio>> = Client::new().token(token).get(AudioGetRecommendations::new().count(200));

    match songs {
        Ok(songs) => print_m3u(&songs),
        Err(ClientError::Api(VkError { error_code: VkErrorCode::Unauthorized, .. })) =>
            find_songs(&fetch_access_token().unwrap(), query, performer_only),
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
        .arg(Arg::with_name("user")
             .short("u")
             .takes_value(true)
             .help("User id"))
        .get_matches();

    let token = get_access_token().unwrap();

    let query = args.value_of("query").unwrap();
    //let lookup_type = if args.is_present("user") { LookUpType::User }
        //else if args.is_present("performer") { LookUpType::Performer }
        //else { LookUpType::Title };

    find_songs(&token, query,
               args.is_present("Performer"));
               //args.value_of("user").and_then(|v| v.parse::<i64>().ok()));
}
