mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

mod console {
    pub fn write_line(s: &str) {
        println!("{}", s);
    }
}

fn main() {
    use console::write_line;
    write_line(&format!("{}", math::add(1, 2)));
}
