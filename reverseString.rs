// Function to reverse a string
fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let original_string = "Hello, world!";
    
    // Reverse the original string and print the result
    let reversed_string = reverse_string(original_string);
    println!("Original string: {}", original_string);
    println!("Reversed string: {}", reversed_string);
}
