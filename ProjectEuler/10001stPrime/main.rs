fn prime(num: &i64) -> bool {
    for x in 2..(num/2)+1 {
        if num % x == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut primes: i16 = 0;
    let mut count: i64 = 2;
    while primes < 10001 {
        if prime(&count) == true {
            primes += 1;
        }
    count += 1;
    }
    println!("The 10001st prime is {}", count-1);
}
