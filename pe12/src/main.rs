fn main() {
    let goal = 500;
    let mut count = 1;
    let mut triangle = 0;
    let mut factors = 0;
    
    loop {
        factors = factor_count(triangle);
        if factors >= goal {
            println!("{} has {} factors", triangle, factors);
        }
        triangle += count;
        count += 1;
    }
}

fn factor_count(number: &mut i32) -> &mut i32 {
    let mut factors = 0;
    let triangle = number;
    for possible_factor in 1..triangle/2 {
        if triangle % possible_factor == 0 {
            factors += 2;
            continue;
        }
    }
    factors
}

