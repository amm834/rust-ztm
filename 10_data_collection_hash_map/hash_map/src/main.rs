use std::collections::HashMap;

fn main() {
    let mut people = HashMap::new();
    people.insert("Ed", 10);
    people.insert("John", 20);

    println!("People = {:?}", people);

    let ed = people.get("Ed");
    if let Some(age) = ed {
        println!("Person age is {}", age);
    } else {
        println!("No info for ed");
    }

    for (person, age) in people {
        println!("{:?} - {}", person, age);
    }
}
