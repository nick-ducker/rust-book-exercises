fn main() {
    let mut x = 5;
    
    x = x + 1;
    {
        let x = x * 2;
        println!("The value of x inside the scope is : {}", x);
    }
    
    println!("The value of x is : {}", x);
}