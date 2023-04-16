fn main() {
    let add = |a, b| a + b;
    let result = add(1, 2);
    println!("{:?}", result);

    let phoneRegx = Regex::new(r"(\d{3})-(\d{3,8})").unwrap();
    let emailRegx = Regex::new(r"(\w+)@(\w+)\.(\w+)").unwrap();
}



