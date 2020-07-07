use chrono::NaiveDate;

type Date = NaiveDate;

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
    let dt = Date::from_ymd(2020, 12, 1);
    let dt = dt.format("%A %d %B").to_string();
    println!("Date: {}", dt);
}
