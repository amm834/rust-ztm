fn main() {
    let a: Option<u32> = Some(5);
    dbg!(a);

    let a = a.map(|num| num + 1);
    dbg!(a);

    let a = a.filter(|num| num == &1);
    dbg!(a);

    let unwarpped = a.unwrap_or_else(|| 0);
    dbg!(unwarpped);
}
