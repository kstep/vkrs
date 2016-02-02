extern crate vkrs;
extern crate clap;
extern crate serde_json as json;

mod helpers;

use helpers::{fetch_access_token, get_access_token};

use clap::{Arg, App};
use vkrs::api::{self, Client, Collection};
use vkrs::auth::AccessToken;
use vkrs::video::{self, Video};

fn print_videos(videos: &[Video]) {
    for video in videos {
        println!("Title: {}", video.title);
        println!("Player: {}", video.player);
        if let Some(ref files) = video.files {
            println!("Files: {:?}", files);
        }
    }
}

fn find_videos(api: &Client, token: &AccessToken, query: &str) -> api::Result<Collection<Video>> {
    api.get(Some(token),
        video::Search::new()
            .q(query)
            .adult(false)
            .count(30))
}

fn main() {
    let args = App::new("videos")
        .author("Konstantin Stepanov <me@kstep.me>")
        .version("0.1.0")
        .about("Search for videos in VK")
        .arg(Arg::with_name("query")
             .required(true)
             .help("Query string"))
        .get_matches();

    let api = Client::new();
    let query = args.value_of("query").unwrap();
    let mut token = get_access_token(&api).unwrap();

    loop {
        let videos = find_videos(&api, &token, query);

        match videos {
            Ok(Collection { ref items, .. }) => return print_videos(items),
            Err(api::Error::Api(api::ApiError { error_code: api::ErrorCode::Unauthorized, .. })) =>
                token = fetch_access_token(&api).unwrap(),
            Err(err) => panic!("Error: {}", err)
        }
    }
}
