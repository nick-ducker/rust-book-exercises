fn main() {
    println!("Hello, world!");

    another_function_boi(5);

    two_param_boi(6, 'k');

    i_am_a_statement();
    let expressed_value = i_am_an_expression();

    println!("This has been expressed: {}", expressed_value);
}

fn two_param_boi(x: i32, unit: char) {
    println!("The measurement is {}{}, I think",x ,unit)
}

fn another_function_boi(param: i32) {
    println!("Oh, hello, its a new function!");

    println!("You gave me the param of {}", param)
}

fn i_am_a_statement() {
    let x = 5;
    println!("Look at me make {} statements", x);
}

fn i_am_an_expression() -> i32 {
    5
}


