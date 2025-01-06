fn types_of_function() {
    // Free function
    println!("The sum of 3 and 4 is {}", add(3, 4));
    // Method function
    let point = Point { x: 5, y: 10 };
    println!(
        "The distance from the origin is {}",
        point.distance_from_origin()
    );
    // Closure function
    let add_two = |x| x + 2;
    println!("3 plus 2 is {}", add_two(3));
    // Generator function
    let mut generator = count_up_from(5);
    println!("{}", generator.next().unwrap());
    println!("{}", generator.next().unwrap());
}

// Free function
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Method function
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn distance_from_origin(&self) -> f64 {
        (self.x.pow(2) + self.y.pow(2)).sqrt()
    }
}
// Closure function

// let add_two = |x| x + 2;
// Generator function
fn count_up_from(start: i32) -> impl Iterator<Item = i32> {
    (start..).into_iter()
}
