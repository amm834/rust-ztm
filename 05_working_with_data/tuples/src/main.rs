enum Emplyee {
    Full,
    Partial,
}

fn main() {
    let (a, b, c) = (1, 2, 3);
    println!("a: {}, b: {}, c: {}", a, b, c);

    let (employee, access) = ("Mg Mg", Emplyee::Full);
}
