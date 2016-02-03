
use serde_json::value::Value;

request_ref! {
    struct Execute for ["execute"](v => 5.44) -> Value {
        code: str = ("") => {}
    }
}
