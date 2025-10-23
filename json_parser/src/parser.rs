#[derive(Debug)]
struct JsonEntry {
    key: String,
    value: JsonValue,
}

#[derive(Debug)]
enum JsonValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Object(Vec<JsonEntry>),
    Array(Vec<JsonValue>),
}
