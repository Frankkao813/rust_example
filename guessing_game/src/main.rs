use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number); // For debugging purposes

    println!("Please input your guess.");
    // Create a mutable String to hold the user's input
    //::new - associated function of the String`type

    // ::stdin() function from the io module
    // read_line returns a Result value, which is a type that can be either Ok or Err




    // The loop keyword creates an infinite loop
    loop{
        
        let mut guess = String::new();
        println!("You guessed: {}", guess);

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // trim the input and parse it to a u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        // match the number
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // When the user wins, it exits the loop using the break keyword
            Ordering::Equal =>{
                println!("You win!");
                break;
            } 
        }
    }

}

