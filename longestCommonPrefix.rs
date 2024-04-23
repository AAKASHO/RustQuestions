// Function to find the longest common prefix of a set of strings
fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() {
        return String::new(); // If the input set of strings is empty, return an empty string
    }

    // Iterate through the characters of the first string
    for (i, c) in strings[0].chars().enumerate() {
        // Check if the character matches the corresponding character in other strings
        for s in &strings[1..] {
            if let Some(sc) = s.chars().nth(i) {
                if sc != c {
                    return strings[0][..i].to_string(); // Return the prefix found so far
                }
            } else {
                return strings[0][..i].to_string(); // Return the prefix found so far
            }
        }
    }

    strings[0].to_string() // If all characters match, return the first string
}

fn main() {
    let strings1 = ["flower", "flow", "flight"];
    let strings2 = ["dog", "racecar", "car"];

    // Find the longest common prefix of strings1 and print the result
    println!("Longest common prefix of strings1: {}", longest_common_prefix(&strings1));

    // Find the longest common prefix of strings2 and print the result
    println!("Longest common prefix of strings2: {}", longest_common_prefix(&strings2));
}
