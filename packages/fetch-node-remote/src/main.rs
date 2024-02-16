use reqwest;
use serde::Deserialize;
use serde_json::Value;
use colored::*;
use semver::Version;

#[derive(Debug, Deserialize)]
pub struct NodeMeta {
    pub version: String,
    pub date: String,
    pub files: Vec<String>,
    pub npm: Option<String>,
    pub v8: String,
    pub uv: Option<String>,
    pub zlib: Option<Value>,
    pub openssl: Option<Value>,
    pub modules: Option<Value>,
    pub lts: Option<Value>,
    pub security: Option<Value>,
}


fn main() {
    let resp = reqwest::blocking::get("https://nodejs.org/dist/index.json")
        .expect("fetch nodejs.org/dist/index.json failed.");

    let json: Vec<NodeMeta> = resp.json().expect("Error");

    let mut json: Vec<_> = json.into_iter().filter_map(|node| {
        let version = Version::parse(&node.version[1..]).unwrap();
        match &node.lts {
            Some(Value::String(lts)) => Some((version, format!("{:<10} {:<12} {}", node.version, node.date.bright_black(), lts.bright_green()))),
            Some(Value::Bool(lts)) if *lts == false => Some((version, format!("{:<10} {:<12}", node.version, node.date.bright_black()))),
            _ => None,
        }
    }).collect();

    json.sort_by(|a, b| a.0.cmp(&b.0));

    for (_, line) in json {
        println!("{}", line);
    }
}
