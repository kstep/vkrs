use std::borrow::Borrow;
use api::{Bool, Collection, FullId, Id, OwnerId, ReportReason, Timestamp, SortOrder};

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
    struct Report for ["photos.report"](v => 5.44) -> Bool [Photos] {
        owner_id: OwnerId = () => {},
        photo_id: Id = () => {},
        reason: ReportReason = () => {AsRef},
    }
}

request! {
    struct ReportComment for ["photos.reportComment"](v => 5.44) -> Bool [Photos] {
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
            sort: Sort = () => {AsRef},
            offset: usize = (0) => {},
            count: usize = (30) => {},
            radius: u16 = (5000) => {},
        }
        unsized {
            q: str = ("") => {=},
        }
    }
}

request_ref! {
    struct Save for ["photos.save"](v => 5.44) -> Vec<Photo> [Photos] {
        sized {
            server: Id = () => {},
            album_id: Option<Id> = () => {Option},
            group_id: Option<Id> = () => {Option},
            latitude: f32 = () => {},
            longitude: f32 = () => {},
        }
        unsized {
            photos_list: str = ("") => {=},
            hash: str = ("") => {=},
            caption: str = ("") => {=},
        }
    }
}

request_ref! {
    struct Copy for ["photos.copy"](v => 5.44) -> Id [Photos] {
        sized {
            owner_id: OwnerId = () => {},
            photo_id: Id = () => {},
        }
        unsized {
            access_key: str = ("") => {=},
        }
    }
}

request_ref! {
    struct Edit for ["photos.edit"](v => 5.44) -> Bool [Photos] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            photo_id: Id = () => {},
            latitude: f32 = () => {},
            longitude: f32 = () => {},
            delete_place: bool = () => {bool},
        }
        unsized {
            caption: str = ("") => {=},
            place_str: str = ("") => {=},
            foursquare_id: str = ("") => {=},
        }
    }
}

request! {
    struct Move for ["photos.move"](v => 5.44) -> Bool [Photos] {
        owner_id: OwnerId = () => {},
        photo_id: Id = () => {},
        target_album_id: Id = () => {},
    }
}

request! {
    struct MakeCover for ["photos.makeCover"](v => 5.44) -> Bool [Photos] {
        owner_id: Option<OwnerId> = () => {Option},
        photo_id: Id = () => {},
        album_id: Id = () => {},
    }
}

request! {
    struct ReorderAlbums for ["photos.reorderAlbums"](v => 5.44) -> Bool [Photos] {
        owner_id: Option<OwnerId> = () => {Option},
        album_id: Id = () => {},
        before: Option<Id> = () => {Option},
        after: Option<Id> = () => {Option},
    }
}

request! {
    struct ReorderPhotos for ["photos.reorderPhotos"](v => 5.44) -> Bool [Photos] {
        owner_id: Option<OwnerId> = () => {Option},
        photo_id: Id = () => {},
        before: Option<Id> = () => {Option},
        after: Option<Id> = () => {Option},
    }
}

request! {
    struct GetAll for ["photos.getAll"](v => 5.45, photo_sizes => 1) -> Collection<Photo> [Photos] {
        owner_id: Option<OwnerId> = () => {Option},
        extended: bool = () => {bool},
        offset: usize = (0) => {},
        count: usize = (20) => {},
        no_service_albums: bool = () => {bool},
        need_hidden: bool = () => {bool},
        skip_hidden: bool = () => {bool},
    }
}

request! {
    struct GetUserPhotos for ["photos.getUserPhotos"](v => 5.45, photo_sizes => 1) -> Collection<Photo> {
        user_id: Option<Id> = () => {Option},
        extended: bool = () => {bool},
        offset: usize = (0) => {},
        count: usize = (20) => {},
        sort: Sort = () => {AsRef},
    }
}

request! {
    struct DeleteAlbum for ["photos.deleteAlbum"](v => 5.44) -> Bool [Photos] {
        album_id: Id = () => {},
        group_id: Option<Id> = () => {Option},
    }
}

request! {
    struct Delete for ["photos.delete"](v => 5.44) -> Bool [Photos] {
        photo_id: Id = () => {},
        owner_id: Option<OwnerId> = () => {Option},
    }
}

request! {
    struct ConfirmTag for ["photos.confirmTag"](v => 5.44) -> Bool [Photos] {
        owner_id: Option<OwnerId> = () => {Option},
        photo_id: Id = () => {},
        tag_id: Id = () => {},
    }
}

request_ref! {
    struct GetComments for ["photos.getComments"](v => 5.44, extended => 0) -> Collection<Comment> [Photos] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            photo_id: Id = () => {},
            need_likes: bool = () => {bool},
            start_comment_id: Option<Id> = () => {Option},
            offset: usize = (0) => {},
            count: usize = (20) => {},
            sort: SortOrder = () => {},
        }
        unsized {
            access_key: str = ("") => {=},
            //fields: [PhotoCommentField] = (&[][..]) => {Vec}, // TODO
        }
    }
}

request! {
    struct GetAllComments for ["photos.getAllComments"](v => 5.44) -> Collection<Comment> [Photos] {
        owner_id: Option<OwnerId> = () => {Option},
        album_id: Option<Id> = () => {},
        need_likes: bool = () => {bool},
        offset: usize = (0) => {},
        count: usize = (20) => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct CreateComment for ["photos.createComment"](v => 5.44) -> Id [Photos] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            photo_id: Id = () => {},
            from_group: bool = () => {bool},
            reply_to_comment: Option<Id> = () => {Option},
            sticker_id: Option<Id> = () => {Option},
            guid: Option<Id> = () => {Option},
        }
        unsized {
            message: str = ("") => {=},
            attachments: [Attachment] = (&[][..]) => {Vec},
            access_key: str = ("") => {=},
        }
    }
}

request! {
    struct DeleteComment for ["photos.deleteComment"](v => 5.44) -> Bool [Photos] {
        owner_id: Option<OwnerId> = () => {Option},
        comment_id: Id = () => {},
    }
}

request! {
    struct Restore for ["photos.restore"](v => 5.44) -> Bool [Photos] {
        owner_id: OwnerId = () => {},
        photo_id: Id = () => {},
    }
}

request! {
    struct RestoreComment for ["photos.restoreComment"](v => 5.44) -> Bool [Photos] {
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

impl Default for Sort {
    fn default() -> Sort {
        Sort::Popularity
    }
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
