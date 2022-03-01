use std::io;

fn main() {
    let mut running = true;
    
    while running {
        let mut nth = String::new();
    
        println!("Enter the number of fibz you want");
        io::stdin().read_line(&mut nth)
            .expect("failed to read input");
        let nth: String = match nth.trim().parse() { 
            Ok(num) => num,
            Err(_) => {
                println!("Sorry m8, not a number");
                continue;
            },
        };
        
        println!("you entered {}", nth);
        running = false;
    }
}
