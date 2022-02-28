fn main() {
    let looping = true;

    if looping {
        let mut count = 0;

        // ' before a variable denotes a label, cool.
        'counting_up: loop {
            println!("Count = {}", count);
            let mut remaining = 10;

            loop {
                println!("Remaining loops = {}", remaining);
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            };

            count +=1;
        }
        println!("End count = {}", count);
        //////////////////////////////////////////////
        let mut new_counter = 0;

        let result = loop {
            new_counter += 1;

            if new_counter == 10 {
                break new_counter * 2;
            }
        };

        println!("The result of the expressive loop is {}", result);
        //////////////////////////////////////////////
        let mut countdown_num = 5;

        while countdown_num != 0 {
            println!("{}!", countdown_num);
            countdown_num -= 1;
        };
        println!("Beep");
        /////////////////////////////////////////////
        let array = [10,20,60,80];
        for element in array {
            println!("{}", element);
        }


    } else {
        let number = 12_0000;
        
        // must be a bool, can't do that whacky JS stuff
        if number < 9001 {
            println!("This is a boring number");
        } else if number > 9000 && number < 10_000 {
            println!("IT'S OVER 9000!");
        } else {
            println!("WREEEEE");
        }
    
        // Oh this is cool
        let cold = true;
        let string = if cold {"brrrr"} else {"def not brrr"};
        println!("Hmm the weather is {}", string);
    }

}
