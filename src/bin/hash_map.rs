use std::collections::HashMap;

#[derive(Debug)]
struct Contents {
    content: String,
}

fn main() {
    let mut people = HashMap::new();
    people.insert("James", 30);
    people.insert("Vesper", 21);
    people.insert("Q", 25);
    people.insert("M", 80);
    people.remove("M");

    println!("Match Person Age: ");
    match people.get("Vesper") {
        Some(age) => println!("age = {:?}", age),
        None => println!("not found"),
    }

    println!("Iterate People: ");
    for (person, age) in people.iter() {
        println!("person = {:?}, age = {:?}", person, age);
    }

    println!("People Keys:");
    for person in people.keys() {
        println!("person = {:?}", person);
    }

    println!("People Values:");
    for age in people.values() {
        println!("age = {:?}", age);
    }

    let mut lockers = HashMap::new();
    lockers.insert(
        1,
        Contents {
            content: "drugs".to_owned(),
        },
    );
    lockers.insert(
        2,
        Contents {
            content: "gun".to_owned(),
        },
    );
    lockers.insert(
        3,
        Contents {
            content: "books".to_owned(),
        },
    );
    println!("Locker Data:");
    for (locker_number, content) in lockers.iter() {
        println!("locker number: {:?}, content: {:?}", locker_number, content);
    }
}
