fn is_palindrome(num: &i32) -> bool {
    let num_string = num.to_string();
    let half = num_string.len()/2;
    return num_string.bytes().take(half).eq(num_string.bytes().rev().take(half));
}

fn main() {
    let mut largest_palindrome = 0;
    let mut x_val = 0;
    let mut y_val = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if is_palindrome(&(x*y)) == true && x*y > largest_palindrome {
                largest_palindrome = x*y;
                x_val = x;
                y_val = y;
            }
        }
        
    }
    println!("The largest palindrome is {} which is the product of {} * {}", largest_palindrome, x_val, y_val);
}
