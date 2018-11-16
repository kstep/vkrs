use std::borrow::Borrow;
use api::{Attachment, Bool, Collection, Comment, FullId, Id, OwnerId, ReportReason, SortOrder, Timestamp};

request_ref! {
    #[derive(Eq, Copy)]
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
    #[derive(Eq, Copy)]
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
    #[derive(Eq, Copy)]
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
    #[derive(Eq, Copy)]
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
    #[derive(Eq, Copy)]
    struct GetAlbumsCount for ["photos.getAlbumsCount"](v => 5.45) -> u32 {
        user_id: Option<Id> = () => {Option},
        group_id: Option<Id> = () => {Option},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
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
    #[derive(Eq, Copy)]
    struct GetUploadServer for ["photos.getUploadServer"](v => 5.45) -> UploadServer [Photos] {
        album_id: Id = () => {},
        group_id: Option<Id> = () => {Option},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetOwnerPhotoUploadServer for ["photos.getOwnerPhotoUploadServer"](v => 5.45) -> UploadUrl {
        owner_id: Option<OwnerId> = () => {Option},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetChatUploadServer for ["photos.getChatUploadServer"](v => 5.45) -> UploadUrl [Messages] {
        chat_id: Id = () => {},
        crop_x: u32 = () => {},
        crop_y: u32 = () => {},
        crop_width: u32 = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetMarketUploadServer for ["photos.getMarketUploadServer"](v => 5.45) -> UploadUrl [Market, Photos] {
        group_id: Id = () => {},
        main_photo: bool = () => {bool},
        crop_x: u32 = () => {},
        crop_y: u32 = () => {},
        crop_width: u32 = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetMarketAlbumUploadServer for ["photos.getMarketAlbumUploadServer"](v => 5.45) -> UploadUrl [Market, Photos] {
        group_id: Id = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetWallUploadServer for ["photos.getWallUploadServer"](v => 5.45) -> UploadServer [Photos] {
        group_id: Option<Id> = () => {Option},
    }
}

request! {
    struct GetMessagesUploadServer for ["photos.getMessagesUploadServer"](v => 5.45) -> UploadServer [Photos];
}

request_ref! {
    #[derive(Eq, Copy)]
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
    #[derive(Eq, Copy)]
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
    #[derive(Eq, Copy)]
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
    #[derive(Eq, Copy)]
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
    #[derive(Eq, Copy)]
    struct SaveMessagesPhoto for ["photos.saveMessagesPhoto"](v => 5.45) -> SavedPhoto [Photos] {
        photo: str = ("") => {=},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct Report for ["photos.report"](v => 5.44) -> Bool [Photos] {
        owner_id: OwnerId = () => {},
        photo_id: Id = () => {},
        reason: ReportReason = () => {AsRef},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct ReportComment for ["photos.reportComment"](v => 5.44) -> Bool [Photos] {
        owner_id: OwnerId = () => {},
        comment_id: Id = () => {},
        reason: ReportReason = () => {AsRef},
    }
}

request_ref! {
    #[derive(Copy)]
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
    #[derive(Copy)]
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
    #[derive(Eq, Copy)]
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
    #[derive(Copy)]
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
    #[derive(Eq, Copy)]
    struct Move for ["photos.move"](v => 5.44) -> Bool [Photos] {
        owner_id: OwnerId = () => {},
        photo_id: Id = () => {},
        target_album_id: Id = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
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
    #[derive(Eq, Copy)]
    struct ReorderPhotos for ["photos.reorderPhotos"](v => 5.44) -> Bool [Photos] {
        owner_id: Option<OwnerId> = () => {Option},
        photo_id: Id = () => {},
        before: Option<Id> = () => {Option},
        after: Option<Id> = () => {Option},
    }
}

request! {
    #[derive(Eq, Copy)]
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
    #[derive(Eq, Copy)]
    struct GetUserPhotos for ["photos.getUserPhotos"](v => 5.45, photo_sizes => 1) -> Collection<Photo> {
        user_id: Option<Id> = () => {Option},
        extended: bool = () => {bool},
        offset: usize = (0) => {},
        count: usize = (20) => {},
        sort: Sort = () => {AsRef},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct DeleteAlbum for ["photos.deleteAlbum"](v => 5.44) -> Bool [Photos] {
        album_id: Id = () => {},
        group_id: Option<Id> = () => {Option},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct Delete for ["photos.delete"](v => 5.44) -> Bool [Photos] {
        photo_id: Id = () => {},
        owner_id: Option<OwnerId> = () => {Option},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct ConfirmTag for ["photos.confirmTag"](v => 5.44) -> Bool [Photos] {
        owner_id: Option<OwnerId> = () => {Option},
        photo_id: Id = () => {},
        tag_id: Id = () => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct GetComments for ["photos.getComments"](v => 5.44, extended => 0) -> Collection<Comment> [Photos] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            photo_id: Id = () => {},
            need_likes: bool = () => {bool},
            start_comment_id: Option<Id> = () => {Option},
            offset: usize = (0) => {},
            count: usize = (20) => {},
            sort: SortOrder = () => {AsRef},
        }
        unsized {
            access_key: str = ("") => {=},
            //fields: [PhotoCommentField] = (&[][..]) => {Vec}, // TODO
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetAllComments for ["photos.getAllComments"](v => 5.44) -> Collection<Comment> [Photos] {
        owner_id: Option<OwnerId> = () => {Option},
        album_id: Option<Id> = () => {Option},
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
    #[derive(Eq, Copy)]
    struct DeleteComment for ["photos.deleteComment"](v => 5.44) -> Bool [Photos] {
        owner_id: Option<OwnerId> = () => {Option},
        comment_id: Id = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct Restore for ["photos.restore"](v => 5.44) -> Bool [Photos] {
        owner_id: OwnerId = () => {},
        photo_id: Id = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct RestoreComment for ["photos.restoreComment"](v => 5.44) -> Bool [Photos] {
        owner_id: OwnerId = () => {},
        comment_id: Id = () => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct EditComment for ["photos.editComment"](v => 5.44) -> Bool [Photos] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            comment_id: Id = () => {},
        }
        unsized {
            message: str = ("") => {=},
            attachments: [Attachment] = (&[][..]) => {Vec},
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct GetTags for ["photos.getTags"](v => 5.44) -> Vec<Tag> [Photos] {
        sized {
            owner_id: Option<OwnerId> = () => {Option},
            photo_id: Id = () => {},
        }
        unsized {
            access_key: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Copy)]
    struct PutTag for ["video.putTag"](v => 5.44) -> Id [Photos] {
        owner_id: Option<OwnerId> = () => {Option},
        user_id: Id = () => {},
        photo_id: Id = () => {},
        x: f32 = () => {},
        y: f32 = () => {},
        x2: f32 = () => {},
        y2: f32 = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct RemoveTag for ["photos.removeTag"](v => 5.44) -> Bool [Photos] {
        tag_id: Id = () => {},
        owner_id: Option<OwnerId> = () => {Option},
        photo_id: Id = () => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetNewTags for ["photos.getNewTags"](v => 5.44) -> Collection<Photo> [Photos] {
        offset: usize = (0) => {},
        count: usize = (20) => {},
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Photo {
    pub id: Id,
    pub album_id: Id,
    pub owner_id: OwnerId,
    pub user_id: Id,
    pub width: u16,
    pub height: u16,
    pub text: String,
    pub date: Timestamp,
    #[serde(default)]
    pub sizes: Vec<Thumb>,
    #[serde(default)]
    pub placer_id: Id,
    #[serde(default)]
    pub tag_id: Id,
    #[serde(default)]
    pub tag_created: Timestamp,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Album {
    pub id: Id,
    pub thumb_id: Id,
    pub owner_id: OwnerId,
    pub title: String,
    pub description: String,
    pub created: Timestamp,
    pub updated: Timestamp,
    pub privacy: u32, // TODO ???
    pub comment_privacy: u32, // TODO ???
    pub size: u32,
    #[serde(default)]
    pub thumb_is_last: Bool,
    #[serde(default)]
    pub can_upload: Bool,
    #[serde(default)]
    pub sizes: Vec<Thumb>,
    pub thumb_src: Option<String>,
    #[serde(rename="type")]
    pub kind: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Thumb {
    pub src: String,
    pub width: u16,
    pub height: u16,
    #[serde(rename="type")]
    pub kind: ThumbKind,
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ThumbKind {
    Prop75 = 's' as u8,
    Prop130 = 'm' as u8,
    Prop604 = 'x' as u8,
    Adapt130 = 'o' as u8,
    Adapt200 = 'p' as u8,
    Adapt320 = 'q' as u8,
    Prop807 = 'y' as u8,
    Prop1280x1024 = 'z' as u8,
    Prop2560x2048 = 'w' as u8,
}

impl ::serde::de::Deserialize for ThumbKind {
    fn deserialize<D: ::serde::de::Deserializer>(d: &mut D) -> Result<ThumbKind, D::Error> {
        struct Visitor;
        impl ::serde::de::Visitor for Visitor {
            type Value = ThumbKind;
            fn visit_str<E: ::serde::de::Error>(&mut self, v: &str) -> Result<ThumbKind, E> {
                use self::ThumbKind::*;
                Ok(match v {
                    "s" => Prop75,
                    "m" => Prop130,
                    "x" => Prop604,
                    "o" => Adapt130,
                    "p" => Adapt200,
                    "q" => Adapt320,
                    "y" => Prop807,
                    "z" => Prop1280x1024,
                    "w" => Prop2560x2048,
                    _ => return Err(::serde::de::Error::invalid_value("album size type"))
                })
            }
        }
        d.deserialize(Visitor)
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct UploadServer {
    pub upload_url: String,
    pub album_id: Id,
    pub user_id: Id,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct UploadUrl {
    pub upload_url: String,
}

// TODO ???
#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct SavedPhoto {
    pub hash: String,
    pub src: String,
    pub src_big: Option<String>,
    pub src_small: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Tag {
    id: Id,
    user_id: Id,
    placer_id: Id,
    tagged_name: String,
    date: Timestamp,
    viewed: Bool,
    x: f32,
    y: f32,
    x1: f32,
    y1: f32,
}

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
