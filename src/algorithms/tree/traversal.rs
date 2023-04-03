pub struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

pub fn preorder_values(root: &TreeNode) -> Vec<i32> {
    let mut preorder_list = vec![];
    let mut stack = vec![];

    stack.push(root);

    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        preorder_list.push(top.value);

        if let Some(right_child) = &top.right {
            stack.push(&right_child);
        }

        if let Some(left_child) = &top.left {
            stack.push(left_child);
        }
    }

    return preorder_list;
}

#[cfg(test)]
mod tests {

    use super::*;

    fn make_tree(
        value: i32,
        left: Option<Box<TreeNode>>,
        right: Option<Box<TreeNode>>,
    ) -> Option<Box<TreeNode>> {
        Some(Box::new(TreeNode { value, left, right }))
    }

    fn make_leaf(value: i32) -> Option<Box<TreeNode>> {
        make_tree(value, None, None)
    }

    #[test]
    fn test_preorder_simple() {
        let tree = make_tree(
            1,
            make_tree(2, make_tree(6, make_leaf(7), make_leaf(8)), make_leaf(9)),
            make_tree(5, make_leaf(3), make_leaf(4)),
        )
        .unwrap();

        assert_eq!(preorder_values(&tree), vec![1, 2, 6, 7, 8, 9, 5, 3, 4]);
    }
}
