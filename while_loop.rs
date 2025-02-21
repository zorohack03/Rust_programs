fn main() {
    let mut number: i32 = 1;

    while number <= 20 {
        if number % 2 != 0 {
            println!("{}", number);
        }
        number += 1;
    }
}

