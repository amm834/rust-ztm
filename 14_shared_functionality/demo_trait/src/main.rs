trait People {
    fn say(&self);
    fn eat(&self);
}

struct Human;

impl People for Human {
    fn say(&self) {
        println!("Human say");
    }

    fn eat(&self) {
        println!("Human eat");
    }
}

struct Dog;
impl People for Dog {
    fn say(&self) {
        println!("woff")
    }

    fn eat(&self) {
        println!("woff is eating")
    }
}

fn behave(human: impl People) {
    human.say();
}

fn main() {
    let human = Human;
    let dog = Dog;
    behave(human);
    behave(dog);
}
