fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false; // 0 and 1 are not prime numbers
    }
    // Check if num is divisible by any number from 2 to its square root
    for i in 2..=num.sqrt() {
        if num % i == 0 {
            return false; // If num is divisible, it's not prime
        }
    }
    true // If num is not divisible by any number, it's prime
}

fn main() {
    let num1 = 17;
    let num2 = 21;

    // Check if num1 is prime and print the result
    println!("Is {} prime? {}", num1, is_prime(num1));

    // Check if num2 is prime and print the result
    println!("Is {} prime? {}", num2, is_prime(num2));
}