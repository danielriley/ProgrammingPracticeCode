fn main() {
    let mut sum: i32 = 0;
    let mut square_sum: i32 = 0;

    for x in 0..101 {
        sum += x;
        square_sum += x*x;
    }

    let square_difference = (sum*sum) - square_sum;

    println!("{}", square_difference);
}
