use std::io;
use std::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Gen new rng using the local thread and OS, then gen
    // a rand number betwee 0 and 101 (non inclusive of 101)
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input a number");
    
        // Define mutable variable and assign the output String::new()
        // ie, an emtpy string
        let mut guess = String::new();
        
        // read input of user and write it to the *mutable reference*
        // of the guess variable. Returns a Result type, calls expect
        // to crash program on err
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
    
        // Shadow guess var by calling trim, parse and expect.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }
    
        println!("You guessed: {}", guess);
    
        // Match the result of guess.cmp([comparible ref])
        // which returns and Ordering::[enumeration] type,
        // then println accordingly and break loop if correct.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small");
            Ordering::Greater => println!("Too Large");
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}