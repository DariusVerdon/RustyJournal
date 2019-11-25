
struct Village<'a> {
    name: String,
    population: &'a i32,
    gold: &'a i32,
    silver: &'a i32,
    copper: &'a i32,
    deceased: &'a i32,
    sick: &'a i32,
    homeless: &'a i32,
    employed: &'a i32,
}

fn main() {
    println!("Hello, world!");
}
