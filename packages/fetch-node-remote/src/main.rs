use std::collections::hash_map::Values;
use chrono::{DateTime, Utc};
use colored::*;
use reqwest;
use semver::{Version,VersionReq};
use serde::Deserialize;
use serde_json::{Value, Map, json, to_string_pretty};
use std::collections::HashMap;

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
#[derive(Debug)]
pub struct NodeScheduleH {
    pub start: String,
    pub end: String,
    pub version: String,
}

fn fetch_node_schedule() -> Vec<NodeScheduleH>{
    let resp = reqwest::blocking::get(
        "https://raw.githubusercontent.com/nodejs/Release/main/schedule.json",
    )
        .expect("fetch nodejs.org/dist/index.json failed.");
    let res: Value =
        resp.json::<Value>().expect("Error");


    let mut v: Vec<NodeScheduleH> = Vec::new();

    if let Some(json) = res.as_object() {
        json.keys().for_each(|key| {
            if let Some(item) = json.get(key) {

                let version = &key.as_str()[1..];

                v.push(
                    NodeScheduleH {
                        start: item.get("start").unwrap().to_string(),
                        end: item.get("end").unwrap().to_string(),
                        version: version.to_string(),
                    }
                );

            }

        });
    }

    v.sort_by(|a, b| a.end.cmp(&b.end).reverse());

    v
}

fn fetch_node_versions() -> Vec<(Version, String)> {
    let resp = reqwest::blocking::get("https://nodejs.org/dist/index.json")
        .expect("fetch nodejs.org/dist/index.json failed.");

    let json: Vec<NodeMeta> = resp.json().expect("Error");

    let mut json: Vec<_> = json
        .into_iter()
        .filter_map(|node| {
            let version = Version::parse(&node.version[1..]).unwrap();
            match &node.lts {
                Some(Value::String(lts)) => Some((
                    version,
                    format!(
                        "{:<10} {:<12} {}",
                        node.version,
                        node.date.bright_black(),
                        lts.bright_green()
                    ),
                )),
                Some(Value::Bool(lts)) if *lts == false => Some((
                    version,
                    format!("{:<10} {:<12}", node.version, node.date.bright_black()),
                )),
                _ => None,
            }
        })
        .collect();

    json.sort_by(|a, b| a.0.cmp(&b.0));

    json
}

fn main() {
    let json = fetch_node_schedule();
    json.iter().for_each(|node| {
        println!("{:?}", node);
    });

    // let json = fetch_node_versions();
    // for (_, line) in json {
    //     println!("{}", line);
    // }
}
