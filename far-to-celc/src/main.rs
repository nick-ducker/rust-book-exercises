use std::io;

fn main() {
    let mut running = true;
    while running {
        let mut measurement = String::new();

        println!("Are we converting to C or F?");
        io::stdin().read_line(&mut measurement)
            .expect("Error reading input");
        let measurement = measurement.trim();

        if measurement == "C" {
            let mut celcius = String::new();
            println!("Input the degrees celcius to convert");
            io::stdin().read_line(&mut celcius)
                .expect("Error reading input");

            let celcius = celcius
                .trim()
                .parse()
                .expect("Could not parse value into integer");

            let result =  convert_to_fahrenheit(celcius);

            println!("This converts to: {}F", result);

        } else if measurement == "F" {
            let mut fahrenheit = String::new();
            println!("Input the fahreneight to convert");
            io::stdin().read_line(&mut fahrenheit)
                .expect("Error reading input");
            let fahrenheit: i32 = fahrenheit
                .trim()
                .parse()
                .expect("Could not parse into an integer");

            let result =  convert_to_celcius(fahrenheit as f32);

            println!("This converts to: {}c", result);
        } else {
            println!("Thats not C or F");
            continue;
        }

        running = false;
    }
    let fahrenheit = 104.00;

    let converted_fahrenheit = convert_to_celcius(fahrenheit);

    let converted_celcius = convert_to_fahrenheit(converted_fahrenheit);

    println!("we start with {}F, which converts to {}c, and back to {}F", fahrenheit, converted_fahrenheit, converted_celcius);
}

fn convert_to_celcius(fahrenheit: f32) -> i32 {
    let celcius = ((fahrenheit - 32.0) * 5.0) / 9.0;
    celcius as i32
}

fn convert_to_fahrenheit(celcius: i32) -> f32 {
    let doubled_celc = (celcius * 2) as f32;
    let fahrenheit = (doubled_celc - doubled_celc * 0.1) + 32 as f32;
    fahrenheit
}
