struct Package {
    weight: u32,
}

impl Package {
    fn new(weight: u32) -> Self {
        Self {
            weight,
        }
    }
}

impl Default for Package {
    fn default() -> Self {
        Self {
            weight: 0,
        }
    }
}

fn main() {
    let package = Package::default();
    println!("{}", package.weight);
}
