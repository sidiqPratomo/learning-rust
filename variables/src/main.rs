mod const_vs_immutable;
mod const_vs_variable;


fn main() {
    let mut x = 5;
    println!("The value of x is :{}", x);
    x = 6;
    println!("The value of x is :{}", x);

    const SUBSCRIBER_COUNT: u32 = 100000;

    let tuple = ("Hello world", 100_000);
    let (text, subscriber) = tuple;

    let subscriber = tuple.1;
    println!("tuple {}", subscriber);

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    println!("error {}", not_found);

    let byte = [0; 8];

    let sum = my_function(100, 200);
    println!("the sum is {}", sum);

    looping();

    while_loop();

    for_loop();
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("the value x is {}", x);
    println!("the value y is {}", y);
    // let sum = x+y;
    // return sum;
    x + y
}

fn looping() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The result is {}", result);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("Lift off!!!!")
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..4 {
        println!("the number is {}", number);
    }
}
