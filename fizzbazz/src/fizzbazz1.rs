pub fn run() {
    let mut x = 1;
    while x <= 100 {
        if x % 15 == 0 {
            println!("FizzBazz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Bazz");
        } else {
            println!("{}", x);
        }
        x += 1;
    }
}
