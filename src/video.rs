use std::borrow::Borrow;
use std::convert::AsRef;
use std::string::ToString;
use std::error::Error;
use api::{Bool, Collection, Duration, FullId, Id, LikesCount, OwnerId, Sort, Timestamp};

#[cfg(feature = "unstable")]
include!("video.rs.in");

#[cfg(not(feature = "unstable"))]
include!(concat!(env!("OUT_DIR"), "/video.rs"));

request_ref! {
    #[derive(Eq, Copy)]
    struct Get for ["video.get"](v => 5.44) -> Collection<Video> [Video] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            album_id: Option<Id> = () => {Option},
            offset: usize = (0) => {},
            count: usize = (30) => {},
        }
        unsized {
            videos: [FullId] = (&[][..]) => { |value|
                &*value.iter().map(|&(o, id)| format!("{}_{}", o, id)).collect::<Vec<_>>().join(",")
            }
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct Search for ["video.search"](v => 5.44) -> Collection<Video> [Video] {
        sized {
            sort: Sort = (Sort::Popularity) => {AsRef},
            hd: bool = () => {bool},
            adult: bool = () => {bool},
            search_own: bool = () => {bool},
            longer: Option<usize> = () => {Option},
            shorter: Option<usize> = () => {Option},
            offset: usize = (0) => {},
            count: usize = (30) => {},
        }
        unsized {
            filters: [Filter] = (&[][..]) => {AsRef<Vec>},
            q: str = ("") => {=},
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct Edit for ["video.edit"](v => 5.44) -> Bool [Video] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            video_id: Id = () => {},
            no_comments: bool = () => {bool},
            repeat: bool = () => {bool},
        }
        unsized {
            name: str = ("") => {=},
            desc: str = ("") => {=},
            privacy_view: str = ("") => {=},
            privacy_comment: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct Add for ["video.add"](v => 5.44) -> Bool [Video] {
        target_id: OwnerId = () => {},
        video_id: Id = () => {},
        owner_id: OwnerId = () => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct Save for ["video.save"](v => 5.44) -> SavedVideo [Video] {
        sized {
            is_private: bool = (true) => {bool},
            wallpost: bool = () => {bool},
            group_id: Id = () => {},
            album_id: Id = () => {},
            no_comments: bool = () => {bool},
            repeat: bool = () => {bool},
        }
        unsized {
            name: str = ("") => {=},
            description: str = ("") => {=},
            link: str = ("") => {=},
            privacy_view: str = ("") => {=},
            privacy_comment: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct Delete for ["video.delete"](v => 5.44) -> Bool [Video] {
        video_id: Id = () => {},
        owner_id: Option<OwnerId> = () => {Option},
        target_id: Option<OwnerId> = () => {Option},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct Restore for ["video.restore"](v => 5.44) -> Bool [Video] {
        video_id: Id = () => {},
        owner_id: Option<OwnerId> = () => {Option},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetUserVideos for ["video.getUserVideos"](v => 5.44, extended => 0) -> Collection<Video> [Video] {
        user_id: Option<Id> = () => {Option},
        offset: usize = (0) => {},
        count: usize = (30) => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetAlbums for ["video.getAlbums"](v => 5.44) -> Collection<Album> [Video] {
        owner_id: Option<OwnerId> = () => {Option},
        offset: usize = (0) => {},
        count: usize = (30) => {},
        need_system: bool = () => {bool},
        extended: bool = (true) => {bool},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetAlbumById for ["video.getAlbumById"](v => 5.44) -> Album [Video] {
        owner_id: Option<OwnerId> = () => {Option},
        album_id: Id = () => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct AddAlbum for ["video.addAlbum"](v => 5.44) -> AlbumId [Video] {
        sized {
            group_id: Option<Id> = () => {Option},
            privacy: Privacy = (Privacy::OnlyMe) => {AsRef},
        }
        unsized {
            title: str = ("") => {=},
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct EditAlbum for ["video.editAlbum"](v => 5.44) -> Bool [Video] {
        sized {
            group_id: Option<Id> = () => {Option},
            album_id: Id = () => {},
            privacy: Option<Privacy> = () => {AsRef<Option>},
        }
        unsized {
            title: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct DeleteAlbum for ["video.deleteAlbum"](v => 5.44) -> Bool [Video] {
        group_id: Option<Id> = () => {Option},
        album_id: Id = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct ReorderAlbums for ["video.reorderAlbums"](v => 5.44) -> Bool [Video] {
        group_id: Option<Id> = () => {Option},
        album_id: Id = () => {},
        before: Option<Id> = () => {Option},
        after: Option<Id> = () => {Option},
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Privacy {
    All = 0,
    Friends = 1,
    FriendsOfFriends = 2,
    OnlyMe = 3,
}

impl AsRef<str> for Privacy {
    fn as_ref(&self) -> &str {
        use self::Privacy::*;
        match *self {
            All => "0",
            Friends => "1",
            FriendsOfFriends => "2",
            OnlyMe => "3",
        }
    }
}

enum_str! { Filter {
    YouTube = "youtube",
    Vimeo = "vimeo",
    Short = "short",
    Long = "long",
}}
