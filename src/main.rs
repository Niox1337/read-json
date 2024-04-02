use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

struct Tag{
    color: String,
    name: String,
}

fn main() {
    println!("Hello, world!");
}
