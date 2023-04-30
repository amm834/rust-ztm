#[derive(Debug)]
struct Dimension {
    width: i32,
    height: i32,
    depth: i32,
}

trait Convey {
    fn weight(&self) -> i32;
    fn dimension(&self) -> Dimension;
}

#[derive(Debug)]
struct ConveyOrBelt<T: Convey> {
    pub items: Vec<T>,
}

impl<T: Convey> ConveyOrBelt<T> {
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
}

#[derive(Debug)]
struct CarPart {
    weight: i32,
    height: i32,
    depth: i32,
    car_part: String,
}

impl Convey for CarPart {
    fn weight(&self) -> i32 {
        self.weight
    }

    fn dimension(&self) -> Dimension {
        Dimension {
            height: self.height,
            width: self.weight,
            depth: self.depth,
        }
    }
}

impl Default for CarPart {
    fn default() -> Self {
        Self {
            weight: 10,
            depth: 10,
            height: 10,
            car_part: "123".to_string(),
        }
    }
}


fn main() {
    let mut belt = ConveyOrBelt {
        items: vec![]
    };
    belt.add(CarPart::default());
    belt.add(CarPart {
        car_part: "456".to_string(),
        weight: 20,
        height: 30,
        depth: 144,
    });

    // println!("{:#?}", belt);
    for item in belt.items {
        println!("{:#?}", item.dimension());
    }
}
