// Function to find the kth smallest element in an array
fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}

fn main() {
    let arr = [3, 1, 4, 1, 5, 9, 2, 6, 5];
    let k = 3;

    // Find the 3rd smallest element in the array and print the result
    match kth_smallest(&arr, k) {
        Some(smallest) => println!("The {}rd smallest element is: {}", k, smallest),
        None => println!("Invalid value of k"),
    }
}
