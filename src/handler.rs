use std::collections::HashMap;
use std::sync::LazyLock;

use crate::resp::Value;

fn ping(_args: &[Value]) -> Value {
    Value {
        typ: "string".to_string(),
        str: "PONG".to_string(),
        ..Default::default()
    }
}

type Handler = fn(&[Value]) -> Value;
pub static HANDLERS: LazyLock<HashMap<&'static str, Handler>> = LazyLock::new(|| {
    let mut m: HashMap<&'static str, Handler> = HashMap::new();
    m.insert("PING", ping);
    m
});
