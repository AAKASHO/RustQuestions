fn is_palindrome(input: &str) -> bool {
    // Create a reversed version of the input string
    let reversed: String = input.chars().rev().collect();
    // Compare the original string with the reversed string
    input == reversed
}

fn main() {
    // Test strings
    let test_string1 = "radar";
    let test_string2 = "hello";

    // Check if test strings are palindromes and print the result
    println!("Is '{}' a palindrome? {}", test_string1, is_palindrome(test_string1));
    println!("Is '{}' a palindrome? {}", test_string2, is_palindrome(test_string2));
}