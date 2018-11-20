extern crate vkrs;
extern crate clap;
extern crate serde_json as json;

mod helpers;

use helpers::{fetch_access_token, get_access_token};

use clap::{Arg, App};
use vkrs::auth::AccessToken;
use vkrs::status;
use vkrs::api::{self, Client};

fn main() {
    let args = App::new("set-status")
        .author("Konstantin Stepanov <me@kstep.me>")
        .version("0.1.0")
        .about("Set VK status")
        .arg(Arg::with_name("status")
             .required(true)
             .help("New status"))
        .get_matches();

    let api = Client::new();

    let status = args.value_of("status").unwrap();
    let mut token = get_access_token(&api).unwrap();

    api.call(Some(&token), status::Set::new().text(status)).unwrap();
}
