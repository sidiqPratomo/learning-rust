fn constant() {
    // This is a constant defined with the `const` keyword. 
    // It must be annotated with its type and is defined in the global scope.
    const MAX_POINTS: u32 = 100_000;

    // This is an immutable variable defined with the `let` keyword.
    // It does not need to be annotated with its type, because the Rust compiler can infer it from the value being assigned.
    // It is defined inside a function, so it is not in the global scope.
    let current_points = 50_000;

    // The value of an immutable variable can be read, but it cannot be modified.
    // This line of code would produce an error, because current_points is an immutable variable.
    // current_points = 75_000;
    // However, it is possible to shadow an immutable variable with a new `let` binding with the same name.
    // This creates a new variable that shadows the old one, effectively "re-assigning" the value. 
    let current_points = 75_000;
    // The value of a constant can also be read, but it cannot be modified.
    // This line of code would produce an error, because MAX_POINTS is a constant.
    // MAX_POINTS = 75_000;

    println!("{}",current_points);
    }