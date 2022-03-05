fn main() {
    // This is a hardcoded string literal,
    // This is stored on the stack within the binary =D
    // It is immutable, which makes life a bit harder
    let string_literal = "Hello there";

    println!("{}", string_literal);

    // This is a String that is *built* from a literal
    // This is stored on the heap using a pointer
    // This can be mutated, yay!
    let mut string_typed_string = String::from("Hello here");
    string_typed_string.push_str(" Mr Nick");

    println!("{}", string_typed_string + ", beep");

    // When we do this with data that lives on the stack,
    // The value is copied to the new variable,
    // super easy!
    let x = "Zing";
    let y = x;
    println!("{}, {}",x, y);

    // The value here lives on the heap, and therefore the,
    // variable has a pointer attached to it. When we assign 
    // the value to a new variable, the previous pointer is
    // invalidated and the "heap" variable goes out of scope.
    let heap = String::from("I'm gonna getcha");
    let heaping = heap; // MOVE
    let heaps = heaping.clone();
    // The below println will break
    // println!("{}", heap);

    // the below println is valid as is the one below
    println!("{}", heaping);
    println!("{}, {}", heaping, heaps);

    let the_boi = String::from("the boi");
    // note that the boi must be returned here, or
    // the ownership will go out of scope and
    // we'll get a big sad.
    let (the_boi_2, len) = calculate_length(the_boi);

    println!("{} is {} long",the_boi_2, len);


    // Rust calls drop on heap stored memory after this point
}

// This function takes ownership of the string it operates on
// and then explicitly returns it so the passed parameter doesn't
// go out of scope at the end of the fucntion.
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}