fn main() {
    let number = 12_0000;


    // must be a bool, can't do that whacky JS stuff
    if number < 9001 {
        println!("This is a boring number")
    } else if number > 9000 && number < 10_000 {
        println!("IT'S OVER 9000!")
    } else {
        println!("WREEEEE")
    }
}
