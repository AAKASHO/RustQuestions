// Function to find the shortest word in a string
fn shortest_word(input: &str) -> Option<&str> {
    // Split the input string into words
    let words: Vec<&str> = input.split_whitespace().collect();
    // Find the shortest word using the min_by() function
    words.iter().min_by_key(|&word| word.len()).cloned()
}

fn main() {
    let input_string = "Rust is a systems programming language";
    
    // Find the shortest word in the input string and print the result
    match shortest_word(input_string) {
        Some(shortest) => println!("Shortest word: {}", shortest),
        None => println!("No words found in the input string"),
    }
}
