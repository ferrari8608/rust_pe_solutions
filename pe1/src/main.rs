fn main() {
    let divisors = [3, 5];
    let upper_limit = 1000;
    let mut result = 0;
    
    for number in 1..upper_limit {
        for divisor in divisors.iter() {
            if number % divisor == 0 {
                result += number;
                break;
            }
        }
    }
    
    println!("{}", result);
}
