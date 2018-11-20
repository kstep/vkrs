# vkrs

Vkontakte API Rust client library

[![Join the chat at https://gitter.im/kstep/vkrs](https://badges.gitter.im/kstep/vkrs.svg)](https://gitter.im/kstep/vkrs)
[![Travis CI build status](https://travis-ci.org/kstep/vkrs.svg?branch=master)](https://travis-ci.org/kstep/vkrs)
[![Downloads](https://img.shields.io/crates/d/vkrs.png)](https://crates.io/crates/vkrs)
[![Version](https://img.shields.io/crates/v/vkrs.png)](https://crates.io/crates/vkrs)
[![License MIT/Apache-2.0](https://img.shields.io/crates/l/vkrs.png)](https://crates.io/crates/vkrs)
[![Code coverage at Coveralls](https://img.shields.io/coveralls/kstep/vkrs.png)](https://coveralls.io/github/kstep/vkrs)
[![Issue Stats](http://issuestats.com/github/kstep/vkrs/badge/pr)](http://issuestats.com/github/kstep/vkrs)
[![Issue Stats](http://issuestats.com/github/kstep/vkrs/badge/issue)](http://issuestats.com/github/kstep/vkrs)

[Documentation](http://kstep.me/vkrs/vkrs/index.html)

## Usage

Add to your Cargo.toml:

```toml
[dependencies]
vkrs = "0.6.3"
```

Then [add your app](https://vk.com/apps?act=manage) at [vk.com](https://vk.com/).

Then authorize and use:

```rust
extern crate vkrs;

use std::{env, io};
use vkrs::*;

fn main() {
    let api = api::Client::new();
    let oauth = api.auth(
        env::var("VK_APP_ID").unwrap(),
        env::var("VK_APP_SECRET").unwrap());

    let auth_uri = oauth.auth_uri(auth::Permission::Audio).unwrap();
    // Or if you want to get permissions for specific request:
    // let auth_uri = oauth.auth_uri_for::<audio::Search>();
    println!("Go to {} and enter code below...", auth_uri);

    let inp = io::stdin();
    let code = {
        let mut buf = String::new();
        inp.read_line(&mut buf).unwrap();
        buf
    };

    // You may want to save this token for future use to avoid asking user
    // to authorize the app on each run.
    let token = oauth.request_token(code.trim()).unwrap();

    // The access token is JSON serializable with serde, so you can do it this way:
    // File::create(TOKEN_FILE).ok().map(|mut f| serde_json::to_writer(&mut f, &token).ok()).unwrap();
    //
    // And then you can load it again:
    // let token: auth::AccessToken = File::open(TOKEN_FILE).ok().and_then(|mut f| serde_json::from_reader(&mut f).ok()).unwrap();

    let songs = api.call(Some(&token),
        audio::Search::new()
            .q("Poets Of The Fall")
            .performer_only(true)
            .count(200))
        .unwrap();

    for song in &songs.items {
        println!("{:?}", song);
    }
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
