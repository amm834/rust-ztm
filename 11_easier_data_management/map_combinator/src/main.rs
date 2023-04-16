fn may_be_number() -> Option<u32> {
    Some(10)
}

fn may_be_word() -> Option<String> {
    Some("hello".to_string())
}


fn main() {
    let number = may_be_number().map(|num| num + 1);
    println!("{:?}", number);
    let str = may_be_word()
        .map(|word| word.len())
        .map(|len| len + 1);
    println!("{:?}", str);
}
