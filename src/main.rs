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

fn main() {
    println!("Hello, world!");
}
