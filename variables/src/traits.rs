
//Understanding traits
// A trait is a way to define a set of behaviors that a type can have. 
// a trait is similar to an interface in other programming languages, 
// but it is more powerful because it allows you to define both the behavior 
// and the types of the arguments and return values.

trait Eq {
    fn eq(&self, other: &Self) ->bool;
}

impl Eq for i32 {
    fn eq(&self, other: &i32) -> bool {
        *self == *other
    }
}

fn main() {
    let a: i32 = 10;
    let b: i32 = 10;

    // println!("Are a and b equal? {}", a.eq(&b));
    // assert!(a.eq(&b) == false);
    println!(
        "Are a and b equal? {}",
        Eq::eq(&a, &b) // Menggunakan trait `Eq` yang Anda definisikan
    );

    let p1 = Point{x:0,y:0};
    let p2 = Point{x:3,y:4};
    let d = p1.distance(&p2);
    println!("The distance between p1 and p2 is {}", d);

    let rect = Rectangle{ width:10, height:20};
    print_area(rect);
}

//Implements Traits
struct Point{
    x:i32,
    y:i32,
}

trait Distance{
    fn distance(&self, other: &Self) ->f64;
}

impl Distance for Point{
    fn distance(&self, other: &Self) ->f64 {
        let dx = (other.x - self.y) as f64;
        let dy =  (other.y - self.x) as f64;
        (dx * dx + dy * dy).sqrt()
    }
}

//example 2 implements traits
struct Rectangle{
    width: u32,
    height:u32,
}

trait HasArea{
    fn area(&self) -> u32;
}

impl HasArea for Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
fn print_area<T: HasArea>(shape:T){
    println!("The area is {}", shape.area());
}

// Do’s And Don'ts Of Using Trait
// Following are some tips for using traits in Rust:
// Do:
// Use traits to define common behaviors that can be implemented by multiple types.
// Use associated types to define placeholder types that are associated with the trait and can be used as type parameters in the trait's methods.
// Use type bounds to specify requirements that a type must implement a certain trait in order to be used in a particular context.
// Use the impl keyword to implement a trait for a type.
// Don't:
// Don't define a trait with only one method, as this is equivalent to a type alias. Instead, use a type alias or an associated type.
// Don't use a trait as a type. Instead, use a type that implements the trait.
// Don't use a trait as a function argument or return type. Instead, use a type that implements the trait.
// Don't define a trait with a method that takes self as a mutable reference, as this can lead to confusing and error-prone code.