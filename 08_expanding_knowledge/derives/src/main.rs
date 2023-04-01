#[derive(Debug,Clone,Copy)]
enum Position {
    Worker,
    Manager,
    Director,
}


#[derive(Debug, Clone, Copy)]
struct Person {
    position: Position,
    name: String,
}


fn print_person(person: Person) {
    match person.position {
        Position::Worker => println!("{} is a worker", person.name),
        Position::Manager => println!("{} is a manager", person.name),
        Position::Director => println!("{} is a director", person.name),
    }
}

fn main() {
    let person = Person {
        position: Position::Worker,
        name: String::from("John").clone(),
    };
    println!("{:?}", person);
    println!("{:?}", Position::Manager);
    print_person(person);
    print_person(person);
}
