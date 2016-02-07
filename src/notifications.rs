use api::Bool;

request! {
    struct MarkAsViewed for ["notifications.markAsViewed"](v => 5.44) -> Bool [Notifications];
}
