fn main() {
    let a = 1..=2;
    let b = 1..4;
    println!("{:?}", a.zip(b));

    for ch in 'a'..='z' {
        println!("{}", ch);
    }
}
