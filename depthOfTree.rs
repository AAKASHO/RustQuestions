// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    // Constructor to create a new TreeNode
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

// Function to calculate the maximum depth of a binary tree
fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            // Recursively calculate the maximum depth of left and right subtrees
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            // Return the maximum depth among left and right subtrees, plus 1 for the current node
            1 + left_depth.max(right_depth)
        }
        None => 0, // Return 0 if the tree is empty
    }
}

fn main() {
    // Construct a binary tree
    let mut root = TreeNode::new(3);
    root.left = Some(Box::new(TreeNode::new(9)));
    root.right = Some(Box::new(TreeNode::new(20)));
    root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(15)));
    root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));

    // Calculate the maximum depth of the binary tree and print the result
    println!("Maximum depth of the binary tree: {}", max_depth(Some(Box::new(root))));
}
