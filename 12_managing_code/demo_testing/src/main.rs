fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_to_uppercase() {
        assert_eq!(to_uppercase("hello"), "HELLO");
    }
}
