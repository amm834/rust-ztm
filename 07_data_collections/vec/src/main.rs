fn main() {
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.pop();

    for number in numbers {
        println!("{:?}", number);
    }
}
