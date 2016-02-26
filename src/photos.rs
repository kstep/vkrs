use std::borrow::Borrow;
use api::{Bool, Collection, FullId, Id, OwnerId, ReportReason, Timestamp};

request_ref! {
    struct CreateAlbum for ["photos.createAlbum"](v => 5.45) -> Album [Photos] {
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
    struct EditAlbum for ["photos.editAlbum"](v => 5.45) -> Bool [Photos] {
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

request! {
    struct GetAlbumsCount for ["photos.getAlbumsCount"](v => 5.45) -> u32 {
        user_id: Option<Id> = () => {Option},
        group_id: Option<Id> = () => {Option},
    }
}

request_ref! {
    struct GetById for ["photos.getById"](v => 5.45, photo_sizes => 1) -> Vec<Photo> {
        sized {
            extended: bool = () => {bool},
        }
        unsized {
            photos: [FullId] = (&[][..]) => {Vec},
        }
    }
}

request! {
    struct GetUploadServer for ["photos.getUploadServer"](v => 5.45) -> UploadServer [Photos] {
        album_id: Id = () => {},
        group_id: Option<Id> = () => {Option},
    }
}

request! {
    struct GetOwnerPhotoUploadServer for ["photos.getOwnerPhotoUploadServer"](v => 5.45) -> UploadUrl {
        owner_id: Option<OwnerId> = () => {Option},
    }
}

request! {
    struct GetChatUploadServer for ["photos.getChatUploadServer"](v => 5.45) -> UploadUrl [Messages] {
        chat_id: Id = () => {},
        crop_x: u32 = () => {},
        crop_y: u32 = () => {},
        crop_width: u32 = () => {},
    }
}

request! {
    struct GetMarketUploadServer for ["photos.getMarketUploadServer"](v => 5.45) -> UploadUrl [Market, Photos] {
        group_id: Id = () => {},
        main_photo: bool = () => {bool},
        crop_x: u32 = () => {},
        crop_y: u32 = () => {},
        crop_width: u32 = () => {},
    }
}

request! {
    struct GetMarketAlbumUploadServer for ["photos.getMarketAlbumUploadServer"](v => 5.45) -> UploadUrl [Market, Photos] {
        group_id: Id = () => {},
    }
}

request! {
    struct GetWallUploadServer for ["photos.getWallUploadServer"](v => 5.45) -> UploadServer [Photos] {
        group_id: Option<Id> = () => {Option},
    }
}

request! {
    struct GetMessagesUploadServer for ["photos.getMessagesUploadServer"](v => 5.45) -> UploadServer [Photos];
}

request_ref! {
    struct SaveMarketPhoto for ["photos.saveMarketPhoto"](v => 5.45) -> SavedPhoto [Market, Photos] { // TODO ???
        sized {
            group_id: Option<Id> = () => {Option},
            server: Id = () => {},
        }
        unsized {
            photo: str = ("") => {=},
            hash: str = ("") => {=},
            crop_data: str = ("") => {=},
            crop_hash: str = ("") => {=},
        }
    }
}

request_ref! {
    struct SaveMarketAlbumPhoto for ["photos.saveMarketAlbumPhoto"](v => 5.45) -> SavedPhoto [Market, Photos] { // TODO ???
        sized {
            group_id: Id = () => {},
            server: Id = () => {},
        }
        unsized {
            photo: str = ("") => {=},
            hash: str = ("") => {=},
        }
    }
}

request_ref! {
    struct SaveOwnerPhoto for ["photos.saveOwnerPhoto"](v => 5.45) -> SavedPhoto { // TODO ???
        sized {
            server: Id = () => {}, // TODO string???
        }
        unsized {
            photo: str = ("") => {=},
            hash: str = ("") => {=},
        }
    }
}

request_ref! {
    struct SaveWallPhoto for ["photos.saveWallPhoto"](v => 5.45) -> Vec<Photo> {
        sized {
            user_id: Id = () => {},
            group_id: Id = () => {},
            server: Id = () => {}, // TODO string???
        }
        unsized {
            photo: str = ("") => {=},
            hash: str = ("") => {=},
        }
    }
}

request_ref! {
    struct SaveMessagesPhoto for ["photos.saveMessagesPhoto"](v => 5.45) -> SavedPhoto [Photos] {
        photo: str = ("") => {=},
    }
}

request! {
    struct Report for ["photo.report"](v => 5.44) -> Bool [Photos] {
        owner_id: OwnerId = () => {},
        photo_id: Id = () => {},
        reason: ReportReason = () => {AsRef},
    }
}

request! {
    struct ReportComment for ["photo.reportComment"](v => 5.44) -> Bool [Photos] {
        owner_id: OwnerId = () => {},
        comment_id: Id = () => {},
        reason: ReportReason = () => {AsRef},
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

request! {
    struct Restore for ["photo.restore"](v => 5.44) -> Bool [Photos] {
        owner_id: OwnerId = () => {},
        photo_id: Id = () => {},
    }
}

request! {
    struct RestoreComment for ["photo.restoreComment"](v => 5.44) -> Bool [Photos] {
        owner_id: OwnerId = () => {},
        comment_id: Id = () => {},
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
