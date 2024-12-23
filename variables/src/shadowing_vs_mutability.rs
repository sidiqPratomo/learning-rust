fn shadowing() {
    // This is an immutable variable defined with the `let` keyword.
    let x = 5;
    // This is a mutable variable defined with the `let`
    // keyword and the `mut` keyword. 
    let mut y = 5;
    // The value of an immutable variable cannot be modified.
    // This line of code would produce an error, because x is an immutable variable.
    // x = 10;
    // However, it is possible to shadow the immutable
    // variable x with a new `let` binding with the same name.
    // This creates a new variable that shadows the old one, 
    // effectively "re-assigning" the value.
    let x = 10;
    // The value of a mutable variable can be modified directly.
    y = 10;
    println!("x = {}", x); // prints "x = 10"
    println!("y = {}", y); // prints "y = 10" 
}