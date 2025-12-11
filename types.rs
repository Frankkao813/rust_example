fn main(){
    // Mutable Variables
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Declaring Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    // shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);


    // space 
    let spaces = "    ";
    let _spaces = spaces.len();
    println!("The size of the space is {}", _spaces);


    // datatypes
    // scalar types
    let a: i32 = 42;
    let b: f64 = 3.14;
    let c: u32 = 255;
    let d: bool = true;
    let e: char = 'R';


    // compound types

    // tuple version
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y from tuple is: {}", y);

    // array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    let first = arr[0];
    println!("The first month is: {}", months[0]);
    

    // function
    another_function(5);
    print_labeled_measurement(5, 'h');
    five();

    control_flow_example_if_else(6);
}

fn another_function(x:i32){
    println!("Another function.");
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}


// function with Return Values
fn five() -> i32 {
    5
}

fn control_flow_example_if_else(number: i32){
    if number < 5 {
        println!("Condition was true");
    } else if number == 5 {
        println!("Condition was false");
    }
    else {
        println!("Condition was something else");
    }
}