struct Rectangle {
    width: u32,
    height: u32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("Rectangle area: {}", area(&rect1));

    let direction = Direction::Left;
    match direction {
        Direction::Up => println!("Moving up"),
        Direction::Down => println!("Moving down"),
        Direction::Left => println!("Moving left"),
        Direction::Right => println!("Moving right"),
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
