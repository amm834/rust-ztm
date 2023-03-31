enum Direction {
    UP,
    Down,
    Right,
    Left
}

fn go_to(direction: Direction) {
    match direction {
        Direction::UP => println!("Going UP"),
        Direction::Down => println!("Going Down"),
        Direction::Right => println!("Going Right"),
        Direction::Left => println!("Going Left"),
    }
}

fn main() {
    let direction = Direction::UP;
    go_to(direction);
}
