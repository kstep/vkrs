extern crate vkrs;
extern crate clap;
extern crate serde_json as json;

mod helpers;

use helpers::{fetch_access_token, get_access_token};

use clap::{Arg, App};
use vkrs::auth::AccessToken;
use vkrs::audio::{self, Audio};
use vkrs::api::{self, Client, Collection};

fn print_m3u(songs: &[Audio]) {
    println!("#EXTM3U");
    for song in songs {
        println!("#EXTINF:{},{} - {} ({}_{})", song.duration, song.artist, song.title, song.owner_id, song.id);
        println!("{}", song.url);
    }
}

fn find_songs(api: &Client, token: &AccessToken, query: &str, performer_only: bool) -> api::Result<Collection<Audio>> {
    api.get(Some(token),
        audio::Search::new()
            .q(query)
            .performer_only(performer_only)
            .count(200))

    //api.get(Some(token),
    //  AudioGetRecommendations::new()
    //      .count(200))
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

    let api = Client::new();

    let query = args.value_of("query").unwrap();
    let performer_only = args.is_present("Performer");
    let mut token = get_access_token(&api).unwrap();

    loop {
        let songs = find_songs(&api, &token, query, performer_only);

        match songs {
            Ok(Collection { ref items, .. }) => return print_m3u(items),
            Err(api::Error::Api(api::ApiError { error_code: api::ErrorCode::Unauthorized, .. })) =>
                token = fetch_access_token(&api).unwrap(),
            Err(err) => panic!("Error: {}", err)
        }
    }
}
