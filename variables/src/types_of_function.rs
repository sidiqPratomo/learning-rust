use std::iter;
use rand::Rng;

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
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}
// Closure function

// let add_two = |x| x + 2;
// Generator function
fn count_up_from(start: i32) -> impl Iterator<Item = i32> {
    (start..).into_iter()
}

// Function With An Iterator
fn sum_iterator(iter: impl Iterator<Item = i32>) -> i32 {
    let mut sum = 0;
    for x in iter {
        sum += x;
    }
    sum
}

//Function as arguments
fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x))
}

fn main() {
    let double = |x| x * 2; // Gunakan `let` untuk closure
    let y: i32 = apply_twice(double, 5);
    println!("Hasilnya adalah: {}", y);
    let nested = outer(3);
    println!("Hasilnya nested: {}", nested);
}

//nested function(function within function)
fn outer(x: i32) -> i32 {
    fn inner(z: i32, y: i32) -> i32 {
        z + y
    }
    inner(x, 5)
}

//control flow
fn control_flow(){
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(-5..5);
    if n < 0 {
        println!("negative value");
    }else if n ==0{
        println!("zero")
    }else {
        println!("positive value")
    }
}

//loop statement
fn perulangan(){
    //use loop
    let mut i = 0;
    loop{
        println!("loop {}", i);
        i += 1;
        if i == 5{
            break;
        }
    }

    //while statement
    let mut x = 0;
    while x <=10{
        println!("{}", x);
        x += 1;
    }

    //sum value in array 
    let vals = vec![1,2,3,4,5,6,7,8,9,10];
    let mut j = 0;
    let mut sum = 0;
    let n = vals.len();
    while j<n{
        sum += vals[j];
        j += 1;
    }
    println!("The sum is: {}", sum);
}

fn factorial() {
    let vals = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut factorial = 1; // Inisialisasi faktorial dengan nilai 1
    let mut j = 0;
    let n = vals.len();

    while j < n {
        factorial *= vals[j];
        j += 1;
    }

    println!("The factorial is: {}", factorial);
}

fn factorials(num: u32) -> u64 {
    let mut factorial = 1u64; // Gunakan u64 untuk menangani hasil faktorial besar
    for i in 1..=num {
        factorial *= i as u64; // Pastikan tipe data sesuai
    }
    factorial
}

//for statement
fn print(){
    for i in 1..=10{
        print!("{}", i);
    }
    println!();
    let vals = [1,2,3,4,5,6,7,8,9,10];
    for e in vals {
        print!("{}", e);
    }
    println!();
}

//match expression
fn matched(){
    let grades = ["A","B","C","D","E","F","FX"];
    for grade in grades {
        match grade{
            "A" | "B"| "C"| "D"|"E"|"F" =>println!("passed"),
            "FX" => println!("failed"),
            _ => println!("unknown")
        }
    }
}