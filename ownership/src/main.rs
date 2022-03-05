fn main() {
    // This is a hardcoded string literal,
    // This is stored on the stack within the binary =D
    // It is immutable, which makes life a bit harder
    let string_literal = "Hello there";

    println!("{}", string_literal);

    // This is a String that is *built* from a literal
    // This can be mutated, yay!
    let mut string_typed_string = String::from("Hello here");
    string_typed_string.push_str(" Mr Nick");

    println!("{}", string_typed_string + ", beep");
}
