// Function to find the maximum subarray sum using Kadane's algorithm
fn max_subarray_sum(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut max_sum = nums[0];
    let mut current_sum = nums[0];

    for &num in &nums[1..] {
        current_sum = current_sum.max(0) + num; // Add the current number to the current sum if positive
        max_sum = max_sum.max(current_sum); // Update the maximum sum if necessary
    }

    max_sum
}

fn main() {
    let nums1 = vec![-2, 1, -3, 4, -1, 2, 1, -5, 9];
    let nums2 = vec![-1, -2, -3, -4];

    // Find the maximum subarray sum for nums1 and print the result
    println!("Maximum subarray sum for nums1: {}", max_subarray_sum(&nums1));

    // Find the maximum subarray sum for nums2 and print the result
    println!("Maximum subarray sum for nums2: {}", max_subarray_sum(&nums2));
}
