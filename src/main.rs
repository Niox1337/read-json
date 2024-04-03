use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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
    indexed: Vec<Indexed>
}

fn main() {}
