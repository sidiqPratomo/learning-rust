fn constVar() {
    // This is a constant defined with the `const` keyword. 
    // It must be annotated with its type and is defined in the global scope.
    const PI: f64 = 3.1415926535897932384626433832795028841971693993;
    // This is an immutable variable defined with the `let` keyword.
    // It does not need to be annotated with its type, because the Rust compiler can infer it from the value being assigned.
    // It is defined inside a function, so it is not in the global scope.
    let radius = 2.0;
    // The value of PI cannot be modified, because it is a constant.
    // This line of code would produce an error, because PI is a constant. 
    // PI = 3.14;
    // The value of radius can be read, but it cannot be modified.
    // This line of code would produce an error, because radius is an immutable variable.
    // radius = 2.5;
    // However, it is possible to shadow the immutable variable radius with a new `let` binding with the same name.
    // This creates a new variable that shadows the old one, effectively "re-assigning" the value.
    let radius = 2.5; 
}