use chrono::NaiveDate;
use std::fmt;

type Date = NaiveDate;

struct Item {
    name: String,
    description: String,
    expires_on: Date,
    bought_on: Date,
}

impl Item {
    fn new(name: String, description: String, expires_on: Date, bought_on: Date) -> Self {
        Self {
            name,
            description,
            expires_on,
            bought_on,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} expires on {} bought on {}\n\t{}",
            self.name, self.expires_on, self.bought_on, self.description
        )
    }
}

struct List {
    items: Vec<Item>,
}

impl List {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add(&mut self, new_item: Item) {
        for item in self.items.iter() {
            if new_item.name == item.name {
                panic!("This item name is already used!");
            }
        }

        self.items.push(new_item);
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "oublie...")
    }
}

fn main() {
    let it1 = Item::new(
        String::from("Confiture fraise"),
        String::from("Confiture à la fraise bio Coop naturaplan"),
        Date::from_ymd(2021, 1, 1),
        Date::from_ymd(2020, 1, 1),
    );
    let it2 = Item::new(
        String::from("Confiture abricot"),
        String::from("Confiture à l'abricot bio Coop naturaplan"),
        Date::from_ymd(2021, 1, 1),
        Date::from_ymd(2020, 1, 1),
    );

    let mut l = List::new();
    l.add(it1);
    l.add(it2);

    println!("Here's the list: {}", l.items[0]);
}
