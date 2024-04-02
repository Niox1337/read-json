use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

struct Tag{
    color: String,
    name: String,
}

struct Task{
    id: String,
    title: String,
    tags: Vec<Tag>,
    description: String,
    due: String,
    recently_deleted: bool,
    scheduled_deletion: String,
}

fn main() {
    println!("Hello, world!");
}
