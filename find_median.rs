// Function to find the median of a sorted array
fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    // If the array has an odd number of elements, return the middle element
    if len % 2 == 1 {
        arr[len / 2] as f64
    } else {
        // If the array has an even number of elements, return the average of the middle two elements
        let mid1 = arr[len / 2 - 1];
        let mid2 = arr[len / 2];
        (mid1 as f64 + mid2 as f64) / 2.0
    }
}

fn main() {
    let sorted_array1 = vec![1, 2, 3, 4, 5];
    let sorted_array2 = vec![1, 2, 3, 4, 5, 6];

    // Find the median of sorted_array1 and print the result
    println!("Median of array 1: {}", find_median(&sorted_array1));

    // Find the median of sorted_array2 and print the result
    println!("Median of array 2: {}", find_median(&sorted_array2));
}
