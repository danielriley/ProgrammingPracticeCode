fn main() {
    let mut total: i32 = 0;
    let mut last: i32 = 1;
    let mut store: i32 = 0;
    let mut current: i32 = 1;
    while current <= 4000000 {
        if current % 2 == 0 {
            total += current;
        }
        store = current;
        current += last;
        last = store;
    }
    println!("{}", total);
}
