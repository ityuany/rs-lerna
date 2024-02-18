use chrono::{NaiveDate, Utc};
use colored::*;
use reqwest;
use semver::{Version, VersionReq};
use serde::Deserialize;
use serde_json::{json, to_string_pretty, Map, Value};
use std::collections::hash_map::Values;
use std::collections::HashMap;
use std::fmt::format;

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
pub enum Lts {
    Str(String),
    Bool(bool),
}

#[derive(Debug)]
pub struct NodeMeta1 {
    version: String,
    date: String,
    lts: Lts,
}

#[derive(Debug)]
pub struct NodeSchedule {
    pub start: String,
    pub end: String,
    pub version: String,
}

fn fetch_node_schedule() -> Vec<NodeSchedule> {
    let resp = reqwest::blocking::get(
        "https://raw.githubusercontent.com/nodejs/Release/main/schedule.json",
    )
    .expect("fetch nodejs.org/dist/index.json failed.");
    let res: Value = resp.json::<Value>().expect("Error");

    let mut v: Vec<NodeSchedule> = Vec::new();

    if let Some(json) = res.as_object() {
        json.keys().for_each(|key| {
            if let Some(item) = json.get(key) {
                let version = &key.as_str()[1..];

                v.push(NodeSchedule {
                    start: item.get("start").unwrap().to_string(),
                    end: item.get("end").unwrap().to_string(),
                    version: version.to_string(),
                });
            }
        });
    }

    v.sort_by(|a, b| a.end.cmp(&b.end).reverse());

    v
}

fn fetch_node_versions() -> Vec<NodeMeta1> {
    let resp = reqwest::blocking::get("https://nodejs.org/dist/index.json")
        .expect("fetch nodejs.org/dist/index.json failed.");

    let json: Vec<NodeMeta> = resp.json().expect("Error");

    let mut vv: Vec<NodeMeta1> = Vec::new();

    let mut json: Vec<_> = json
        .into_iter()
        .map(|node| NodeMeta1 {
            version: node.version[1..].to_string(),
            date: node.date,
            lts: match &node.lts {
                Some(Value::String(lts)) => Lts::Str(lts.to_string()),
                Some(Value::Bool(lts)) => Lts::Bool(*lts),
                _ => Lts::Bool(false),
            },
        })
        .collect();

    json.sort_by(|a, b| {
        let av = Version::parse(&a.version).expect("Error");
        let bv = Version::parse(&b.version).expect("Error");
        av.cmp(&bv)
    });

    json
}

fn main() {
    // let sss = String::from("2014-07-31");

    // let s = NaiveDate::parse_from_str(&sss, "%Y-%m-%d").expect("parse end date error");

    // println!("{:?}", s);

    // let s = VersionReq::parse("9").unwrap();

    // let a = Version::parse("0.9.0").unwrap();

    // println!("{}", s.matches(&a));

    let schedule_vec = fetch_node_schedule();

    let find_schedule = |item: &NodeMeta1| {
        schedule_vec.iter().find(|schedule| {
            VersionReq::parse(&schedule.version)
                .unwrap()
                .matches(&Version::parse(&item.version).unwrap())
        })
    };

    let node_version_vec = fetch_node_versions();
    for item in node_version_vec {
        let schedule = find_schedule(&item);

        let (is_active, end) = if let Some(schedule) = schedule {
            let end_date =
                NaiveDate::parse_from_str(&schedule.end.trim_matches('"'), "%Y-%m-%d").unwrap();
            (
                end_date > Utc::now().date_naive(),
                format!(" {}", schedule.end.trim_matches('"')),
            )
        } else {
            (false, "".to_string())
        };

        let is_lts = match item.lts {
            Lts::Str(_) => true,
            Lts::Bool(_) => false,
        };

        let lts_str = match item.lts {
            Lts::Str(s) => {
                if is_active {
                    s.green().to_string()
                } else {
                    s
                }
            }
            _ => "".to_string(),
        };

        // let v = if is_active {
        //     item.version
        // } else {
        //     item.version.strikethrough().bright_black().to_string()
        // };

        let min_width = if is_lts { 10 } else { 0 };

        let line = format!("v{:<min_width$} {} {}", item.version, end, lts_str);

        if is_active {
            println!("{}", line);
        } else {
            println!("{}", line.strikethrough().bright_black().to_string());
        }

        // println!("{}", line);
    }
}
