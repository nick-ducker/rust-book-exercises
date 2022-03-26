#[derive(Debug)]
enum State {
    Sa,
    Nt,
    Nsw,
    Qld,
    Wa,
    Tas,
    Vic,
}

enum Coin {
    Five,
    Ten,
    Twenty,
    Fifty(State),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Five => 1,
        Coin::Ten => 5,
        Coin::Twenty => 10,
        Coin::Fifty(state) => {
            println!("this fiddy cent apparently is from {:?}", state);
            25
        },
    }
}
 
fn main() {

}
