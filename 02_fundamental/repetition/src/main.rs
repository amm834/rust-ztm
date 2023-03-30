fn main() {
    let mut a = 0;
    loop {
        if a == 5 {
            break;
        }
        println!("{:?}", a);
        a += 1;
    }

    let mut b = 0;
    while b != 10 {
        println!("{:?}", b);
        b += 1;
    }
}
