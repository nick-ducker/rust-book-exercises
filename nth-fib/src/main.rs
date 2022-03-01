use std::io;

fn main() {
    let mut running = true;
    
    while running {
        let mut nth = String::new();
    
        println!("Enter the number of fibz you want");
        io::stdin().read_line(&mut nth)
            .expect("failed to read input");

        let new_nth: i32 = match nth.trim().parse() { 
            Ok(num) => num,
            Err(_) => {
                println!("Sorry m8, not a number");
                continue;
            },
        };

        println!("Here we go....");
        let mut fibnum = 0;

        if new_nth == 1 {
            fibnum = 0;
        } else if new_nth == 2 {
            fibnum = 1;
        } else {
            let mut fib_arr = [0,1];
            let mut count = 2;
            while count < new_nth {
                fibnum = fib_arr[0] + fib_arr[1];
                fib_arr[0] = fib_arr[1];
                fib_arr[1] = fibnum;
                count += 1;
            }
  
        }; 

        println!("you entered {}", nth);
        println!("This equals {}", fibnum);
        running = false;
    }
}
