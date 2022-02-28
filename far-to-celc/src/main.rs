fn main() {
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
