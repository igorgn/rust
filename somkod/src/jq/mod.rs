use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

pub struct Jq {
    // file_path: PathBuf,
    json_raw: String,
}

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

impl Display for JsonEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{{}: {}}}", self.key, self.value)
    }
}
impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::Boolean(b) => write!(f, "{}", b),
            JsonValue::Null => write!(f, "null"),
            _ => todo!(), // JsonValue::Object(entries) => {
                          //     let entries_str: Vec<String> = entries.iter().map(|e| format!("{}", e)).collect();
                          //     write!(f, "{{{}}}", entries_str.join(", "))
                          // },
                          // JsonValue::Array(values) => {
                          //     let values_str: Vec<String> = values.iter().map(|v| format!("{}", v)).collect();
                          //     write!(f, "[{}]", values_str.join(", "))
                          // },
        }
    }
}

impl From<String> for JsonEntry {
    fn from(value: String) -> Self {
        todo!()
    }
}
impl Jq {
    pub fn new(file_path: PathBuf) -> Self {
        let mut buf = String::new();
        let mut file = File::open(&file_path).unwrap();
        file.read_to_string(&mut buf).unwrap();

        Self {
            // file_path,
            json_raw: buf.trim().to_string(),
        }
    }

    pub fn parse(&self) {
        let result = parse_json(&self.json_raw).unwrap();
    }
}

fn parse_json(json_raw: &str) -> Result<(), String> {
    if !is_valid_json_object(json_raw) {
        return Err("Invalid JSON object".to_string());
    }

    if extract_object(json_raw) == "" {
        // println!("{{}}");
        return Ok(());
    }
    let entries = get_entries(json_raw)?;

    // for e in entries {
        // println!("{}", e);
    // }
    println!("{:?}", entries);
    Ok(())
}

fn is_valid_json_object(json_raw: &str) -> bool {
    json_raw.starts_with('{') && json_raw.ends_with('}')
}
fn get_entries<'a>(json_raw: &str) -> Result<Vec<JsonEntry>, String> {
    let mut entries = Vec::new();
    let trimmed = extract_object(json_raw);
    let entries_raw: Vec<String> = trimmed.split(',').map(|s| s.to_string()).collect();
    for entry_raw in entries_raw {
        let parts: Vec<&str> = entry_raw.split(':').collect();

        if !is_valid_string(parts[0].trim()) {
            return Err("Invalid Key".to_string());
        }

        entries.push(JsonEntry {
            key: parts[0].to_string(),
            value: parse_value(parts[1].trim())?,
        });
    }
    Ok(entries)
}

fn extract_object(json_raw: &str) -> &str {
    json_raw.trim_start_matches('{').trim_end_matches('}')
}

fn is_valid_string(s: &str) -> bool {
    println!("{s}");
    s.starts_with('"') && s.ends_with('"')
}

fn parse_value(value: &str) -> Result<JsonValue, String> {
    let trimmed = value.trim();
    let res = match trimmed {
        "true" => JsonValue::Boolean(true),
        "false" => JsonValue::Boolean(false),
        "null" => JsonValue::Null,
        s if is_valid_string(s) => JsonValue::String(s.to_string()),
        num => num.parse::<f64>().map(JsonValue::Number).map_err(|_| format!("Invalid Value")).unwrap(),
        _ => return Err("Invalid Value".to_string()),
    };
    Ok(res)
    // let mut json_value: JsonValue = None;
    // if is_valid_string(value) {
    //     json_value = JsonValue::String(value.('"').to_string());
    // }

    // if value == "true" {
    //     json_value = JsonValue::Boolean(true);
    // }

    // if value == "false" {
    //     json_value = JsonValue::Boolean(false);
    // }

    // if value == "null" {
    //     json_value = JsonValue::Null
    // }

    // if let Ok(num ) = value.parse::<f64>() {
    //     json_value = JsonValue::Number(num);
    // }

    // if is_valid_json_object(value) {
    //     let entries = get_entries(value)?;
    //     json_value = JsonValue::Object(entries);
    // }

    // if json_value.is_not_initialized() {

    //     return Err("Invalid Value".to_string());
    // }
    // Ok(json_value)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn empty_test() {
        let jq = Jq::new(PathBuf::from("src/jq/tests/step1/valid.json"));
        jq.parse();
    }

    #[test]
    fn fail_empty_test() {
        let jq = Jq::new(PathBuf::from("src/jq/tests/step1/invalid.json"));
        // if let Err(e) = jq.parse() {
        // assert_eq!("Invalid JSON format".to_string(), e)
        // }
    }
}
