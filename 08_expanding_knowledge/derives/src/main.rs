#[derive(Debug)]
enum PromoDiscount {
    NewUser,
    Holiday(String),
}

#[derive(Debug)]
enum Discount {
    Percentage(i32),
    Amount(i32),
    Promotion(PromoDiscount),
}

fn main() {
    let discount = Discount::Promotion(
        PromoDiscount::NewUser
    );
    println!("{:?}", discount);
}
