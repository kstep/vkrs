use users::UserOptionField;
use api::{Bool, Collection, Id, LikesCount, OwnerId, Timestamp};

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct WallPost {
    id: Id,
    owner_id: OwnerId,
    from_id: OwnerId,
    date: Timestamp,
    text: String,
    reply_owner_id: OwnerId,
    reply_post_id: Id,
    friends_only: Bool,
    comments: CommentsCount,
    likes: LikesCount,
    reposts: RepostsCount,
    post_type: PostType,
    post_source: PostSource, // TODO
    attachments: Vec<Attachment>, // TODO
    geo: GeoLocation, // TODO
    signer_id: Id,
    copy_history: Option<Vec<RepostInfo>>, // TODO
    can_pin: Bool,
    is_pinned: Bool,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone, Copy)]
pub struct CommentsCount {
    count: u32,
    can_post: Bool,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone, Copy)]
pub struct RepostsCount {
    count: u32,
    user_reposted: Bool,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone, Copy)]
pub enum PostType {
    Post,
    Copy,
    Reply,
    Postpone,
    Suggest
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone, Copy)]
pub struct PostSource;
#[derive(Debug, PartialEq, Eq, Deserialize, Clone, Copy)]
pub struct Attachment;
#[derive(Debug, PartialEq, Eq, Deserialize, Clone, Copy)]
pub struct GeoLocation;
#[derive(Debug, PartialEq, Eq, Deserialize, Clone, Copy)]
pub struct RepostInfo;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone, Copy)]
pub struct PostId {
    post_id: Id,
}

request_ref! {
    struct Get for ["wall.get"](v => 5.44, extended => 0) -> Collection<WallPost> {
        sized {
            owner_id: OwnerId = () => {},
            filter: Filter = (Filter::All) => {AsRef},
            offset: usize = (0) => {},
            count: usize = (100) => {},
        }
        unsized {
            domain: str = ("") => {=},
            fields: [UserOptionField] = (&[][..]) => {AsRef<Vec>},
        }
    }
}

request_ref! {
    struct Post for ["wall.post"](v => 5.44) -> PostId [Wall] {
        sized {
            owner_id: OwnerId = () => {},
            friend_only: bool = (true) => {bool},
            from_group: bool = (false) => {bool},
            signed: bool = (false) => {bool},
            publish_date: Timestamp = (0) => {},
            lat: f32 = () => {},
            long: f32 = () => {},
            place_id: Id = () => {},
            post_id: Id = () => {},
        }
        unsized {
            message: str = ("") => {=},
            services: str = ("") => {=},
        }
    }
}

enum_str! { Filter {
    Owner = "owner",
    Others = "others",
    All = "all",
    Suggests = "suggests",
}}
