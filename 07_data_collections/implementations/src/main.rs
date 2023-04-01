struct Temperature {
    fahrenheit: f32,
}

impl Temperature {
    fn freezing() -> Self {
        Self {
            fahrenheit: 32.0,
        }
    }

    fn boiling() -> Self {
        Self {
            fahrenheit: 212.0,
        }
    }

    fn display_f(&self) {
        println!("Degree {:?} F", self.fahrenheit);
    }
}

fn main() {
    let temp = Temperature {
        fahrenheit: 32.0,
    };
    temp.display_f();

    let boil = Temperature::boiling();
    boil.display_f();
}
