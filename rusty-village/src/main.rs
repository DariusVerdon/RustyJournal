extern crate rustc_serialize;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Village {
    name: String,
    population: i32,
    gold: i32,
    silver: i32,
    copper: i32,
    deceased: i32,
    sick: i32,
    homeless: i32,
    employed: i32,
}

fn main() {
    let mut village: Village = Village {name: "Elora".to_string(), population: 12, gold: 4, silver: 2, copper: 54, deceased: 0, sick: 2, homeless: 1, employed: 7 };

    println!("{}", village.name);

    let encoded = serde_json::to_string(&village).unwrap();

    println!("serialized = {}", encoded);

    let decoded: Village = serde_json::from_str(&encoded).unwrap();

    println!("deserialized = {:?}", decoded);

}
