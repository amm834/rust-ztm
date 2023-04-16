struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: &str, age: i32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_age(&self) -> i32 {
        self.age
    }
}

fn get_person_by_name(people: Vec<Person>, name: String) -> Result<Person, String> {
    for person in people {
        if person.get_name() == name {
            return Ok(person);
        }
    }
    Err("Person not found".to_string())
}


fn main() {
    let people = vec![
        Person::new("John", 20),
        Person::new("Jane", 21),
        Person::new("Jack", 22),
    ];
    let john = get_person_by_name(people, "Joshn".to_string());
    match john {
        Ok(person) => println!("{} is {} years old", person.get_name(), person.get_age()),
        Err(err) => println!("{}", err),
    }
}
