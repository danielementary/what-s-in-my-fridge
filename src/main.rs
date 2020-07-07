struct Date {
    day: u8,
    month: u8,
    year: u8,
}

struct Item {
    name: String,
    description: String,
    expires_on: Date,
    bought_on: Date,
}

struct List {
    items: Vec<Item>,
}

fn main() {
    println!("Hello, world!");
}
