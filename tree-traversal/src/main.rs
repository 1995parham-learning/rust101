//! Traverse a binary tree in in-order / pre-order / post-order.
//!
//! Type choices:
//! - `val: i64` — Rust's 64-bit signed integer (the draft's `int64` isn't a
//!   real type name).
//! - Children are `Option<Box<Node>>`, not `Option<&Node>`. A `&Node` only
//!   *borrows* a node owned elsewhere; a tree needs to *own* its children, and
//!   `Box` is an owning heap pointer. `Box` is also required for the type to
//!   exist at all: a `Node` holding a bare `Node` would be infinitely sized,
//!   and the fixed-size `Box` pointer breaks that recursion.
//! - `Option` distinguishes an empty subtree (`None`) from a present child.

#[derive(Debug)]
struct Node {
    val: i64,
    // `Box` owns the child on the heap; `Option` allows an absent subtree.
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn leaf(val: i64) -> Box<Node> {
        Box::new(Node {
            val,
            left: None,
            right: None,
        })
    }

    fn new(val: i64, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node { val, left, right })
    }
}

/// Root, then left subtree, then right subtree.
fn pre_order(root: &Option<Box<Node>>, out: &mut Vec<i64>) {
    if let Some(node) = root {
        out.push(node.val);
        pre_order(&node.left, out);
        pre_order(&node.right, out);
    }
}

/// Left subtree, then root, then right subtree.
fn in_order(root: &Option<Box<Node>>, out: &mut Vec<i64>) {
    if let Some(node) = root {
        in_order(&node.left, out);
        out.push(node.val);
        in_order(&node.right, out);
    }
}

/// Left subtree, then right subtree, then root.
fn post_order(root: &Option<Box<Node>>, out: &mut Vec<i64>) {
    if let Some(node) = root {
        post_order(&node.left, out);
        post_order(&node.right, out);
        out.push(node.val);
    }
}

/// Small helper so callers don't have to allocate the output vector themselves.
fn collect(root: &Option<Box<Node>>, order: fn(&Option<Box<Node>>, &mut Vec<i64>)) -> Vec<i64> {
    let mut out = Vec::new();
    order(root, &mut out);
    out
}

fn main() {
    //         1
    //        / \
    //       2   3
    //      / \
    //     4   5
    let tree = Some(Node::new(
        1,
        Some(Node::new(2, Some(Node::leaf(4)), Some(Node::leaf(5)))),
        Some(Node::leaf(3)),
    ));

    println!("pre-order:  {:?}", collect(&tree, pre_order));
    println!("in-order:   {:?}", collect(&tree, in_order));
    println!("post-order: {:?}", collect(&tree, post_order));

    let empty: Option<Box<Node>> = None;
    println!("empty:      {:?}", collect(&empty, in_order));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_tree_yields_nothing() {
        let empty: Option<Box<Node>> = None;
        assert!(collect(&empty, pre_order).is_empty());
        assert!(collect(&empty, in_order).is_empty());
        assert!(collect(&empty, post_order).is_empty());
    }

    #[test]
    fn single_root() {
        let tree = Some(Node::leaf(1));
        assert_eq!(collect(&tree, pre_order), [1]);
        assert_eq!(collect(&tree, in_order), [1]);
        assert_eq!(collect(&tree, post_order), [1]);
    }

    #[test]
    fn root_with_left_child() {
        let tree = Some(Node::new(1, Some(Node::leaf(2)), None));
        assert_eq!(collect(&tree, pre_order), [1, 2]);
        assert_eq!(collect(&tree, in_order), [2, 1]);
        assert_eq!(collect(&tree, post_order), [2, 1]);
    }

    #[test]
    fn root_with_right_child() {
        let tree = Some(Node::new(1, None, Some(Node::leaf(2))));
        assert_eq!(collect(&tree, pre_order), [1, 2]);
        assert_eq!(collect(&tree, in_order), [1, 2]);
        assert_eq!(collect(&tree, post_order), [2, 1]);
    }

    #[test]
    fn root_left_then_right() {
        // root -> left -> right
        let tree = Some(Node::new(1, Some(Node::new(2, None, Some(Node::leaf(3)))), None));
        assert_eq!(collect(&tree, pre_order), [1, 2, 3]);
        assert_eq!(collect(&tree, in_order), [2, 3, 1]);
        assert_eq!(collect(&tree, post_order), [3, 2, 1]);
    }

    #[test]
    fn full_tree() {
        let tree = Some(Node::new(
            1,
            Some(Node::new(2, Some(Node::leaf(4)), Some(Node::leaf(5)))),
            Some(Node::leaf(3)),
        ));
        assert_eq!(collect(&tree, pre_order), [1, 2, 4, 5, 3]);
        assert_eq!(collect(&tree, in_order), [4, 2, 5, 1, 3]);
        assert_eq!(collect(&tree, post_order), [4, 5, 2, 3, 1]);
    }
}
