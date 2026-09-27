use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

use crate::resp::Value;

static DB: LazyLock<RwLock<HashMap<String, String>>> = LazyLock::new(|| {
    let m: HashMap<String, String> = HashMap::new();
    RwLock::new(m)
});

static HDB: LazyLock<RwLock<HashMap<String, HashMap<String, String>>>> = LazyLock::new(|| {
    let m: HashMap<String, HashMap<String, String>> = HashMap::new();
    RwLock::new(m)
});

fn ping(args: &[Value]) -> Value {
    if args.len() == 0 {
        return Value {
            typ: "string".to_string(),
            str: "PONG".to_string(),
            ..Default::default()
        };
    }
    Value {
        typ: "string".to_string(),
        str: args[0].bulk.clone(),
        ..Default::default()
    }
}

fn set(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value {
            typ: "error".to_string(),
            str: "ERR wrong number of arguments for 'set' command".to_string(),
            ..Default::default()
        };
    }

    let key = args[1].bulk.clone();
    let value = args[2].bulk.clone();

    DB.write().unwrap().insert(key, value);

    Value {
        typ: "string".to_string(),
        str: "OK".to_string(),
        ..Default::default()
    }
}

fn get(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value {
            typ: "error".to_string(),
            str: "ERR wrong number of arguments for 'get' command".to_string(),
            ..Default::default()
        };
    }

    if let Some(value) = DB.read().unwrap().get(&args[1].bulk) {
        return Value {
            typ: "string".to_string(),
            str: value.clone(),
            ..Default::default()
        };
    };

    Value {
        typ: "error".to_string(),
        str: "ERR trying to get value from the database".to_string(),
        ..Default::default()
    }
}

fn hset(args: &[Value]) -> Value {
    if args.len() != 3 {
        return Value {
            typ: "error".to_string(),
            str: "ERR wrong number of arguments fo 'hset' command".to_string(),
            ..Default::default()
        };
    }

    let hash = args[0].bulk.clone();
    let key = args[1].bulk.clone();
    let value = args[2].bulk.clone();

    HDB.write()
        .unwrap()
        .entry(hash)
        .or_default()
        .insert(key, value);

    Value {
        typ: "string".to_string(),
        str: "OK".to_string(),
        ..Default::default()
    }
}

fn hget(args: &[Value]) -> Value {
    if args.len() != 2 {
        return Value {
            typ: "error".to_string(),
            str: "ERR wrong number of arguments fo 'hset' command".to_string(),
            ..Default::default()
        };
    }

    let hash = &args[0].bulk;
    let key = &args[1].bulk;

    return match HDB.read().unwrap().get(hash) {
        Some(m) => m
            .get(key)
            .map(|v| Value {
                typ: "string".to_string(),
                str: v.clone(),
                ..Default::default()
            })
            .unwrap_or_else(|| Value::null()),
        None => Value::null(),
    };
}

fn hgetall(args: &[Value]) -> Value {
    if args.len() != 1 {
        return Value {
            typ: "error".to_string(),
            str: "ERR wrong number of arguments for 'hgetall' command".to_string(),
            ..Default::default()
        };
    }
    let hash = &args[0].bulk;

    let array: Vec<Value> = HDB
        .read()
        .unwrap()
        .get(hash)
        .map(|m| {
            m.iter()
                .flat_map(|(k, v)| {
                    [
                        Value {
                            typ: "string".to_string(),
                            str: k.clone(),
                            ..Default::default()
                        },
                        Value {
                            typ: "string".to_string(),
                            str: v.clone(),
                            ..Default::default()
                        },
                    ]
                })
                .collect()
        })
        .unwrap_or_default();

    Value {
        typ: "array".to_string(),
        array,
        ..Default::default()
    }
}

type Handler = fn(&[Value]) -> Value;
pub static HANDLERS: LazyLock<HashMap<&'static str, Handler>> = LazyLock::new(|| {
    let mut m: HashMap<&'static str, Handler> = HashMap::new();
    m.insert("PING", ping);
    m.insert("SET", set);
    m.insert("GET", get);
    m.insert("HSET", hset);
    m.insert("HGET", hget);
    m.insert("HGETALL", hgetall);
    m
});
