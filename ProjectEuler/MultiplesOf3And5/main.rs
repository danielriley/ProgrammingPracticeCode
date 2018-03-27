fn main() {
    let mut total: i32 = 0;
    for x in 1..1000 {
        if (x % 5 == 0 || x % 3 == 0) {
            total += x;
        }
    }
    println!("{}", total)
}
