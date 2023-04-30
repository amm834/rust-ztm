trait Move {
    fn move_to(&self, x: i32, y: i32);
}

struct Grasshopper;

impl Move for Grasshopper {
    fn move_to(&self, x: i32, y: i32) {
        println!("Grasshopper moved to ({}, {})", x, y);
    }
}

struct Fish;

impl Move for Fish {
    fn move_to(&self, x: i32, y: i32) {
        println!("Fish moved to ({}, {})", x, y)
    }
}

fn move_thing<T>(thing: T, a: i32, b: i32)
    where T: Move + Sized,
{
    thing.move_to(a, b)
}

fn main() {
    move_thing(Grasshopper, 10, 20);
    move_thing(Fish, 30, 40);
}
