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

fn main() {}
