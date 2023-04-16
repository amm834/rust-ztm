enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let user: Option<String> = None;
    if let Some(user) = user {
        println!("{}", user);
    } else {
        println!("No user");
    }

    if let Color::Red = Color::Red {
        println!("Red");
    } else {
        println!("Not Red")
    }
}
