enum Drink {
    Tea,
    Coffee,
    Milk,
}


fn main() {
    let drink_type = Drink::Tea;
    let drink = match drink_type {
        Drink::Tea => "tea",
        Drink::Coffee => "coffee",
        Drink::Milk => "milk",
    };
    println!("drink: {}", drink);
}
