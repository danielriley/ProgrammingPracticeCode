use std::process;

fn prime(num: &i64) -> bool {
    for x in 2..num/2 {
        if num % x == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let num: i64 = 600851475143;
    let half: i64 = num/2;
    let mut largest_prime: i64 = 0;
    let mut factor_total: i64 = 1;
    for x in 1..half {
        if prime(&x) == true && num % x == 0 {
            largest_prime = x;
            println!("{} is a prime factor, total is {}", largest_prime, factor_total);
            factor_total = factor_total * largest_prime;
            if factor_total == num {
                println!("The largest prime factor is {}", largest_prime);
                process::exit(1);
            }
        }
    }
}
