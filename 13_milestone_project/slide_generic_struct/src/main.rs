trait Seat {
    fn show(&self);
}

enum SeatType {
    Concert,
    Special,
}

impl Seat for SeatType {
    fn show(&self) {
        match self {
            SeatType::Concert => println!("Concert"),
            SeatType::Special => println!("Special"),
        }
    }
}

struct Plane<T: Seat> {
    location: T,
}

fn print_show<T: Seat>(plane: Plane<T>) {
    plane.location.show();
}

fn main() {
    let plane = Plane {
        location: SeatType::Concert,
    };
   print_show(plane);
}
