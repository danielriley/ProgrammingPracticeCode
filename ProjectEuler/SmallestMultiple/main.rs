fn evenly_divisible_by_20(num: &i32) -> bool{
    for x in 1..21 {
        if num % x != 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut finished = false;
    let mut count: i32 = 0;
    while finished == false {
        count += 20;
        if evenly_divisible_by_20(&count) == true {
            finished = true;
        }
    }
    println!("The smallest number divisible by 1 to 20 is {}", count);
}
