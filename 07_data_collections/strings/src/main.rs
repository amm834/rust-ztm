use std::mem::discriminant;

struct LineCount {
    name: String,
    count: u32,
}

fn print_name(name: &str) {
    println!("count: {:?}", name);
}

fn main() {
    let lines = vec![
        LineCount {
            name: "a".to_string(),
            count: 1,
        },
        LineCount {
            name: "b".to_string(),
            count: 2,
        },
        LineCount {
            name: "c".to_string(),
            count: 3,
        },
    ];


    for line in lines {
        print_name(&line.name);
        println!("count: {}", line.count);
    }
}