use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

pub struct Jq {
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
        write!(f, "{}: {}", self.key, self.value)
    }
}
impl Display for JsonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonValue::String(s) => write!(f, "\"{}\"", s),
            JsonValue::Number(n) => write!(f, "{}", n),
            JsonValue::Boolean(b) => write!(f, "{}", b),
            JsonValue::Null => write!(f, "null"),
            JsonValue::Object(entries) => {
                let entries_str: Vec<String> = entries.iter().map(|e| format!("{}", e)).collect();
                write!(f, "{{{}\n}}", entries_str.join(","))
            }
            JsonValue::Array(values) => {
                let values_str: Vec<String> = values.iter().map(|v| format!("{}", v)).collect();
                write!(f, "[{}]", values_str.join(", "))
            }
        }
    }
}

// impl From<String> for JsonEntry {
//     fn from(value: String) -> Self {
//         todo!()
//     }
// }
impl Jq {
    pub fn new(file_path: PathBuf) -> Self {
        let mut buf = String::new();
        let mut file = File::open(&file_path).unwrap();
        file.read_to_string(&mut buf).unwrap();

        Self {
            json_raw: buf.trim().to_string(),
        }
    }

    pub fn parse(&self) -> Result<(), String> {
        parse_json(&self.json_raw)
    }
}

fn get_entries(json_raw: &str) -> Result<Vec<JsonEntry>, String> {
    let mut entries = Vec::new();
    let internal_object = extract_object(json_raw);
    if internal_object.is_empty() {
        return Ok(entries);
    }
    let entries_raw: Vec<String> = internal_object.split(',').map(|s| s.to_string()).collect();
    for entry_raw in entries_raw {
        let parts: Vec<&str> = entry_raw.split(':').collect();

        if !is_valid_string(parts[0]) {
            eprintln!("Problematic {}", parts[0]);
            return Err("Invalid Key".to_string());
        }

        entries.push(JsonEntry {
            key: parts[0].to_string(),
            value: parse_value(parts[1])?,
        });
    }
    Ok(entries)
}

fn parse_json(json_raw: &str) -> Result<(), String> {
    if !is_valid_json_object(json_raw) {
        return Err("Invalid JSON object".to_string());
    }

    if extract_object(json_raw).is_empty() {
        println!("{{}}");
        return Ok(());
    }
    let entries = JsonValue::Object(get_entries(json_raw)?);

    println!("{}", entries);
    Ok(())
}

fn is_valid_json_object(json_raw: &str) -> bool {
    json_raw.starts_with('{') && json_raw.ends_with('}')
}
fn extract_object(json_raw: &str) -> &str {
    json_raw.trim_start_matches('{').trim_end_matches('}')
}

fn is_valid_string(s: &str) -> bool {
    // println!("{s}");
    s.trim().starts_with('"') && s.trim().ends_with('"')
}

fn parse_value(value: &str) -> Result<JsonValue, String> {
    let res = match value.trim() {
        "true" => JsonValue::Boolean(true),
        "false" => JsonValue::Boolean(false),
        "null" => JsonValue::Null,
        s if is_valid_string(s) => JsonValue::String(s.trim_matches('"').to_string()),
        obj if is_valid_json_object(obj) => {
            // let obj = extract_object(obj);
            let entries = get_entries(obj)?;
            JsonValue::Object(entries)
        }
        // array => {},
        num => num
            .parse::<f64>()
            .map(JsonValue::Number)
            .map_err(|_| "Invalid Value".to_string())?,
        // _ => return Err("Invalid Value".to_string()),
    };
    Ok(res)
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
