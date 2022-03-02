// https://www.codewars.com/kata/5544c7a5cb454edb3c000047/rust

fn bouncing_ball(h: f64,  bounce: f64,  window: f64) -> i32 {
    if h > 0.0 && bounce > 0.0 && bounce < 1.0 && window < h {
        let mut pass = 1;
        let mut apex_height = h * bounce;

        loop {
            if apex_height <= window {
                break;                
            }
            apex_height *= bounce;
            pass += 2;
        }
        return pass.try_into().unwrap();
    }
    return -1;
}