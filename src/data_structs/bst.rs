#![allow(dead_code)]

use std::fmt::Display;

type TreeNode<K, V> = Option<Box<Node<K, V>>>;

#[derive(Debug)]
pub struct Node<K, V> {
    left: TreeNode<K, V>,
    right: TreeNode<K, V>,
    key: K,
    value: V,
}

pub trait BinaryTree {
    fn pre_order(&self);
    fn in_order(&self);
    fn post_order(&self);
}

pub trait BinarySearchTree<K: PartialOrd, V>: BinaryTree {
    fn insert(&mut self, key: K, value: V);
}

impl<K: Display, V: Display> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Node {
            left: None,
            right: None,
            key,
            value,
        }
    }
}

impl<K: PartialOrd + Display, V: Display> BinarySearchTree<K, V> for Node<K, V> {
    fn insert(&mut self, key: K, value: V) {
        if self.key < key {
            if let Some(ref mut right) = self.right {
                right.insert(key, value)
            } else {
                self.right = Some(Box::new(Node::new(key, value)))
            }
        } else {
            if let Some(ref mut left) = self.left {
                left.insert(key, value);
            } else {
                self.left = Some(Box::new(Node::new(key, value)));
            }
        }
    }
}

impl<K: Display, V: Display> BinaryTree for Node<K, V> {
    fn pre_order(&self) {
        println!("{} - {}", self.key, self.value);

        if let Some(ref left) = self.left {
            left.pre_order();
        }
        if let Some(ref right) = self.right {
            right.pre_order();
        }
    }

    fn in_order(&self) {
        if let Some(ref left) = self.left {
            left.in_order()
        }

        println!("{} - {}", self.key, self.value);

        if let Some(ref right) = self.right {
            right.in_order()
        }
    }

    fn post_order(&self) {
        if let Some(ref left) = self.left {
            left.post_order();
        }
        if let Some(ref right) = self.right {
            right.post_order();
        }
        println!("{} - {}", self.key, self.value);
    }
}
