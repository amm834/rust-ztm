fn print_it(str: &str) {
    println!("{}", str);
}

struct Person {
    name: String,
}

fn main() {
    let str = String::from("Hello, world!");
    print_it(&str);
}
