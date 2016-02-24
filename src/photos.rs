use std::borrow::Borrow;
use api::{Collection, Id, OwnerId, Timestamp, Bool};

request_ref! {
    struct CreateAlbum for ["photos.createAlbum"](v => 5.45) -> Album {
        sized {
            group_id: Option<Id> = () => {Option},
            upload_by_admins_only: bool = () => {bool},
            comments_disabled: bool = () => {bool},
        }
        unsized {
            title: str = ("") => {=},
            description: str = ("") => {=},

            // TODO: better type (comma separated words)
            privacy_view: str = ("") => {=},
            privacy_comment: str = ("") => {=},
        }
    }
}

request_ref! {
    struct EditAlbum for ["photos.editAlbum"](v => 5.45) -> Bool {
        sized {
            album_id: Id = () => {},
            owner_id: Option<OwnerId> = () => {Option},
            upload_by_admins_only: bool = () => {bool},
            comments_disabled: bool = () => {bool},
        }
        unsized {
            title: str = ("") => {=},
            description: str = ("") => {=},

            // TODO: better type (comma separated words)
            privacy_view: str = ("") => {=},
            privacy_comment: str = ("") => {=},
        }
    }
}

request_ref! {
    struct GetAlbums for ["photos.getAlbums"](v => 5.45) -> Collection<Album> {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            need_system: bool = () => {bool},
            need_covers: bool = () => {bool},
            photo_sizes: bool = () => {bool},
            offset: usize = (0) => {},
            count: usize = (100) => {},
        }
        unsized {
            album_ids: [Id] = (&[][..]) => {Vec},
        }
    }
}

request_ref! {
    struct Get for ["photos.get"](v => 5.45, photo_sizes => 1) -> Collection<Photo> {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            album_id: Option<Id> = () => {Option}, // TODO: can be negative or string (wall, profile, saved)
            rev: bool = () => {bool},
            extended: bool = () => {bool},
            feed: Option<Id> = () => {Option},
            offset: usize = (0) => {},
            count: usize = (100) => {},
        }
        unsized {
            photo_ids: [Id] = (&[][..]) => {Vec},
            feed_type: str = ("") => {=},
        }
    }
}

request_ref! {
    struct Search for ["photos.search"](v => 5.37) -> Collection<Photo> {
        sized {
            lat: f32 = () => {},
            long: f32 = () => {},
            start_time: Timestamp = () => {},
            end_time: Timestamp = () => {},
            sort: Sort = (Sort::Popularity) => {AsRef},
            offset: usize = (0) => {},
            count: usize = (30) => {},
            radius: u16 = (5000) => {},
        }
        unsized {
            q: str = ("") => {=},
        }
    }
}

#[cfg(feature = "unstable")]
include!("photos.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/photos.rs"));

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum Sort {
    DateAdded = 0,
    Popularity = 1,
}

impl AsRef<str> for Sort {
    fn as_ref(&self) -> &str {
        use self::Sort::*;
        match *self {
            DateAdded => "0",
            Popularity => "1",
        }
    }
}
