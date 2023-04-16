fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let numbers_1 = numbers
        .iter()
        .map(|num| num + 1)
        .collect::<Vec<_>>();
    println!("{:?}", numbers_1);

    let max = numbers.iter().max();
    match max {
        Some(max) => println!("max is {}", max),
        None => println!("no max"),
    }

    let max = numbers.iter().max().unwrap();
    println!("max is {}", max);

    let max = numbers.iter().fold(0, |acc, x| acc.max(*x));
    println!("max is {}", max);
}
