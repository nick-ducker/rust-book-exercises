fn main() {
    println!("Hello, world!");

    another_function_boi(5);

    two_param_boi(6, 'k');
}

fn two_param_boi(x: i32, unit: char) {
    println!("The measurement is {}{}, I think",x ,unit)
}

fn another_function_boi(param: i32) {
    println!("Oh, hello, its a new function!");

    println!("You gave me the param of {}", param)
}
