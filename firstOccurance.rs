// Function to return the index of the first occurrence of a given number in a sorted array
fn first_occurrence_index(arr: &[i32], target: i32) -> Option<usize> {
    // Initialize variables for left and right indices of the array
    let mut left = 0;
    let mut right = arr.len() - 1;

    // Perform binary search to find the first occurrence of the target number
    while left <= right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            // If the current element is the target, check if it's the first occurrence
            // by verifying if the previous element is different or if it's the first element
            if mid == 0 || arr[mid - 1] != target {
                return Some(mid); // Return the index of the first occurrence
            } else {
                // If not the first occurrence, adjust the search range to the left
                right = mid - 1;
            }
        } else if arr[mid] < target {
            // If the current element is less than the target, search in the right half
            left = mid + 1;
        } else {
            // If the current element is greater than the target, search in the left half
            right = mid - 1;
        }
    }

    None // Return None if the target number is not found in the array
}

fn main() {
    let sorted_array = vec![1, 2, 3, 4, 4, 5, 6, 7, 8];
    let target1 = 5;
    let target2 = 9;

    // Find the index of the first occurrence of target1 in the sorted array
    match first_occurrence_index(&sorted_array, target1) {
        Some(index) => println!("First occurrence of {} is at index {}", target1, index+1),
        None => println!("{} not found in the array", target1),
    }

    // Find the index of the first occurrence of target2 in the sorted array
    match first_occurrence_index(&sorted_array, target2) {
        Some(index) => println!("First occurrence of {} is at index {}", target2, index),
        None => println!("{} not found in the array", target2),
    }
}
