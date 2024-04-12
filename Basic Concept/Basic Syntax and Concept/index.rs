fn main() {
    // Variables and data types
    let x: i32 = 5;
    let y = 10;
    let z = x + y;
    println!("The sum of {} and {} is: {}", x, y, z);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("Maximum points allowed: {}", MAX_POINTS);

    // Control flow
    let number = 7;
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
}
