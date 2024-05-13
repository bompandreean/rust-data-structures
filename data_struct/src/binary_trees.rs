use std::fmt::Debug;

mod binary_tree {
    use std::fmt::Debug;

    #[derive(Debug)]
    pub struct MyBinaryTree<T> {
        root: Option<Box<Node<T>>>,
    }

    #[derive(Debug)]
    pub struct Node<T> {
        data: T,
        left: MyBinaryTree<T>,
        right: MyBinaryTree<T>,
    }

    impl<T: PartialOrd> MyBinaryTree<T> {
        pub fn new() -> MyBinaryTree<T> {
            MyBinaryTree {
                root: None,
            }
        }

        pub fn add_child(&mut self, item: T) {
            match self.root {
                None => {
                    self.root = Some(Box::new(Node {
                        data: item,
                        left: MyBinaryTree::new(),
                        right: MyBinaryTree::new(),
                    }));
                }
                Some(ref mut node) => {
                    if item < node.data {
                        node.left.add_child(item);
                    } else {
                        node.right.add_child(item);
                    }
                }
            }
        }
    }

    impl<T: Debug> MyBinaryTree<T> {
        pub fn print_left_first(&self, depth: usize) {
            if let Some(ref node) = self.root {
                node.left.print_left_first(depth + 1);
                let mut space = String::new();
                for _ in 0..depth {
                    space.push_str("---");
                }
                println!("{}{:?}", space, node.data);
                node.right.print_left_first(depth + 1);
            }
        }
    }
}

mod balanced_binary_tree {
    /*
        A binary tree is balanced if the height of the tree is O(Log n) where n is the number of nodes.
    For Example, the AVL tree maintains O(Log n) height by making sure that the difference between
    the heights of the left and right subtrees is at most 1. Red-Black trees maintain O(Log n) height by
    making sure that the number of Black nodes on every root-to-leaf path is the same and that there are no adjacent red nodes.
    Balanced Binary Search trees are performance-wise good as they provide O(log n) time for search, insert and delete.

        A balanced binary tree is a binary tree that follows the 3 conditions:
    The height of the left and right tree for any node does not differ by more than 1.
    The left subtree of that node is also balanced.
    The right subtree of that node is also balanced.
     */


    use std::fmt::Debug;

    #[derive(Debug)]
    pub struct MyBinaryBalancedTree<T> {
        root: Option<Box<Node<T>>>,
    }

    #[derive(Debug)]
    pub struct Node<T> {
        data: T,
        height: i8,
        left: MyBinaryBalancedTree<T>,
        right: MyBinaryBalancedTree<T>,
    }

    impl<T: PartialOrd> Node<T> {
        fn rotate_left(mut self) -> Box<Self> {
            if let Some(mut res) = self.right.root.take() {
                self.right = MyBinaryBalancedTree::new_root(res.left.root.take());
                self.right.set_height();
                res.left = MyBinaryBalancedTree::new_root(Some(Box::new(self)));
                res.left.set_height();
                res.height = 1 + std::cmp::max(res.left.height(), res.right.height());

                res
            } else {
                Box::new(self)
            }
        }

        fn rotate_right(mut self) -> Box<Self> {
            if let Some(mut result) = self.left.root.take() {
                self.left = MyBinaryBalancedTree::new_root(result.right.root.take());
                self.left.set_height();
                result.right = MyBinaryBalancedTree::new_root(Some(Box::new(self)));
                result.right.set_height();
                result.height = 1 + std::cmp::max(result.left.height(), result.right.height());

                result
            } else {
                Box::new(self)
            }
        }
    }

    impl<T: PartialOrd> MyBinaryBalancedTree<T> {
        pub fn new() -> MyBinaryBalancedTree<T> {
            MyBinaryBalancedTree {
                root: None,
            }
        }

        pub fn new_root(root: Option<Box<Node<T>>>) -> MyBinaryBalancedTree<T> {
            MyBinaryBalancedTree {
                root,
            }
        }

        pub fn height(&self) -> i8 {
            match self.root {
                Some(ref node) => node.height,
                _ => 0
            }
        }
        pub fn set_height(&mut self) {
            if let Some(ref mut node) = self.root {
                node.height = std::cmp::max(node.right.height(), node.left.height()) + 1;
            }
        }

        pub fn add_child(&mut self, item: T) {
            let rot_direction = match self.root {
                None => {
                    self.root = Some(Box::new(Node {
                        data: item,
                        height: 0,
                        left: MyBinaryBalancedTree::new(),
                        right: MyBinaryBalancedTree::new(),
                    }));

                    0
                }
                Some(ref mut node) => {
                    if item < node.data {
                        node.left.add_child(item);
                        if node.left.height() - node.right.height() > 1 {
                            1
                        } else {
                            0
                        }
                    } else {
                        node.right.add_child(item);
                        if node.right.height() - node.left.height() > 1 {
                            -1
                        } else { 0 }
                    }
                }
            };
            match rot_direction {
                1 => self.rotate_right(),
                -1 => self.rotate_left(),
                _ => self.set_height()
            }
        }

        pub fn rotate_left(&mut self) {
            // match self.root.take(){
            //     Some(root) => Self::new_root(Some(root.rotate_left())),
            //     None => Self::new()
            // }

            self.root = self.root.take().map(|existing_root| existing_root.rotate_left());
        }

        pub fn rotate_right(&mut self) {
            self.root = self.root.take().map(|existing_root| existing_root.rotate_right());
        }
    }

    impl<T: Debug> MyBinaryBalancedTree<T> {
        pub fn print_left_first(&self, depth: usize) {
            if let Some(ref node) = self.root {
                node.left.print_left_first(depth + 1);
                let mut space = String::new();
                for _ in 0..depth {
                    space.push_str("---");
                }
                println!("{}:{}{:?}", node.height, space, node.data);
                node.right.print_left_first(depth + 1);
            }
        }
    }
}


#[cfg(test)]
mod test {
    use crate::binary_trees::balanced_binary_tree::MyBinaryBalancedTree;
    use crate::binary_trees::binary_tree::MyBinaryTree;

    #[test]
    fn it_works() {
        let mut my_btree = MyBinaryTree::new();
        my_btree.add_child(9);
        my_btree.add_child(3);
        my_btree.add_child(1);
        my_btree.add_child(5);
        my_btree.add_child(19);
        my_btree.add_child(87);
        my_btree.add_child(4);
        my_btree.add_child(32);

        println!("{:?}", my_btree);
        my_btree.print_left_first(0);
    }

    #[test]
    fn it_works_balanced() {
        let mut my_btree = MyBinaryBalancedTree::new();
        my_btree.add_child(9);
        my_btree.add_child(3);
        my_btree.add_child(1);
        my_btree.add_child(5);
        my_btree.add_child(19);
        my_btree.add_child(87);
        my_btree.add_child(4);
        my_btree.add_child(32);

        println!("{:?}", my_btree);
        my_btree.print_left_first(0);

        println!("--------------rotating---------");
        my_btree.rotate_left();
        my_btree.print_left_first(0);
    }

    #[test]
    fn test_balancing() {
        let mut my_bbtree = MyBinaryBalancedTree::new();
        my_bbtree.add_child(30);
        my_bbtree.add_child(66);
        my_bbtree.add_child(38);
        my_bbtree.add_child(49);
        my_bbtree.add_child(95);
        my_bbtree.add_child(3);
        my_bbtree.add_child(0);
        my_bbtree.add_child(107);
        my_bbtree.add_child(42);
        my_bbtree.add_child(5);

        my_bbtree.print_left_first(0);
    }
}