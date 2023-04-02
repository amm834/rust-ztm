struct Person {
    age: Option<i32>,
    name: String,
}

struct Product {
    name: String,
    quantity: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let products = vec![
        Product {
            name: "Apple".to_string(),
            quantity: 10,
        },
        Product {
            name: "Banana".to_string(),
            quantity: 20,
        },
        Product {
            name: "Orange".to_string(),
            quantity: 30,
        },
    ];
    for product in products {
        if product.name == name {
            return Some(product.quantity);
        }
    }
    None
}

fn main() {
    let alice = Person {
        age: Some(30),
        name: "Alice".to_string(),
    };

    let backey = Person {
        age: None,
        name: "Backey".to_string(),
    };

    match backey.age {
        Some(age) => println!("Backey is {} years old", age),
        None => println!("Backey is not telling their age"),
    }

    println!("Alice is {} years old", alice.age.unwrap_or(0));
    println!("Backey is {} years old", backey.age.unwrap_or(0));

    let apple_quantity = find_quantity("Apple");
    println!("Apple quantity: {}", apple_quantity.unwrap_or(0));
}
