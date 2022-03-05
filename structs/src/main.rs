// Object-esque structs
struct Climber {
    active: bool,
    name: String,
    ewbank: usize,
}

// Tuple structs
struct Coords (usize, usize);
struct Colour(usize, usize, usize);

fn main() {
    let climber_1 = build_climber(String::from("Blueys Bob"), 29);

    let similar_climber = Climber {
        name: String::from("Nowra Nelson"), 
        ..climber_1
    };

    // At this point we can still use climber_1 as we have not moved
    // its string value to similar_climber. The bool and usize values
    // are copied, as they live on the stack not the heap.
    // Had we taken the name of climber_1, the variable would now be
    // unusable.

    

}

fn build_climber(name: String, ewbank: usize) -> Climber {
    Climber {
        name,
        ewbank,
        active: true,
    }
}