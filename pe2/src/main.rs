fn main() {
    let upper_limit = 4000000;
    let mut result = 2;
    let mut previous = [1, 2];
    let mut next = previous[0] + previous[1];
    
    loop {
        previous = [previous[1], next];
        if next > upper_limit {
            break;
        } else if next % 2 == 0 {
            result += next;
        }
        next = previous[0] + previous[1];
    }

    println!("{}", result);
}
