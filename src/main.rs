/*
This program is written by Zhixiang Feng (Niox). The json data should come from
the Chrome extension Hawk - Page Indexer
 */


use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize)]
struct Tag{
    color: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Task{
    id: String,
    title: String,
    tags: Vec<Tag>,
    description: String,
    due: String,
    recently_deleted: bool,
    scheduled_deletion: String,
}

#[derive(Serialize, Deserialize)]
struct Note{
    id: String,
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct IndexedItem{
    body: String,
    id: u16,
    title: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct Indexed{
    corpus: Vec<IndexedItem>,
    links: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct File{
    notes: Vec<Note>,
    tasks: HashMap<String, Task>,
    indexed: Vec<Indexed>,
    all_last_titles: HashMap<String, Vec<String>>,
    allowed_regex: Vec<String>,
    allowed_sites: Vec<String>,
    allowed_string_matches: Vec<String>,
    allowed_urls: Vec<String>,
    tags: Vec<Tag>,
}

fn parse_json(file_name: &str) -> File{
    let file = std::fs::read_to_string(file_name).expect("Failed to read file");
    let file: File = serde_json::from_str(&file).expect("Failed to parse json");
    file
}

fn main() {
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read file name");
}
