enum Discount {
    Percentage(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("one"),
        other => println!("{}", other),
    }


    let discount = Discount::Flat(13);
    match discount {
        Discount::Flat(10) => println!("10"),
        Discount::Flat(n) => println!("${}", n),
        _ => (),
    }

    let ticket = Ticket {
        event: "Concert".to_string(),
        price: 10,
    };

    match ticket {
        Ticket { price: 10, event } => println!("event @ 10 =  {:?}", event),
        Ticket { price, .. } => println!("price: {}", price),
    }
}
