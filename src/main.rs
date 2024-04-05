/*
This program is written by Zhixiang Feng (Niox). The json data should come from
the Chrome extension Hawk - Page Indexer
 */


use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Tag{
    tagColour: String,
    tagName: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Task{
    id: String,
    title: String,
    tags: Vec<String>,
    description: String,
    due: String,
    recentlyDeleted: bool,
    scheduledDeletion: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Note{
    id: String,
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct IndexedItem{
    body: String,
    id: u16,
    title: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Indexed{
    corpus: Vec<IndexedItem>,
    links: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct File{
    notes: Vec<Note>,
    tasks: HashMap<String, Task>,
    indexed: Indexed,
    allLastTitles: HashMap<String, Vec<String>>,
    allowedRegex: Vec<String>,
    allowedSites: Vec<String>,
    allowedStringMatches: Vec<String>,
    allowedURLs: Vec<String>,
    tags: HashMap<String, Tag>,
}

fn parse_json(file_name: &str) -> File{
    let file = std::fs::read_to_string(file_name).expect("Failed to read file");
    let file: File = serde_json::from_str(&file).expect("Failed to parse json");
    file
}

fn main() {
    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read file name");
    let file_name = file_name.trim();
    let file = parse_json(file_name);
    println!("{:?}", file);
}
