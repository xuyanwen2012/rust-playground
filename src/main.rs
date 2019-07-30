#![allow(dead_code)]

mod algorithms;
mod data_structs;

use crate::algorithms::a_star::{Grid, Point};
use crate::data_structs::*;

mod solution;

fn main() {
    let world = Grid::new();
    println!("{}", world);

    let p1 = Point::new(2, 0);
    let p2 = Point::new(3, 0);

    let neighbours = world.get_adjacent(p1);

    let path = world.shortest_path(p1, p2);

    match path {
        Ok(vec) => println!("{:?}", vec),
        Err(e) => println!("{}", e),
    }

    //    println!("{:?}", reverse(to_list(vec![1, 2, 3, 4, 5])));
    //
    //    println!(
    //        "{:?}",
    //        add_two_numbers(to_list(vec![1, 2, 3, 4, 5]), to_list(vec![1, 2, 3, 4, 5]))
    //    );
    //    println!(
    //        "{:?}",
    //        add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4]))
    //    )
    //    {
    //        use bst::*;
    //        let mut root = Node::new(5, 100);
    //        root.insert(2, 101);
    //        root.insert(7, 102);
    //
    //        println!("{:?}", root);
    //
    //        println!("Pre Order traversal");
    //        root.pre_order();
    //        println!("In Order traversal");
    //        root.in_order();
    //        println!("Pos Order traversal");
    //        root.post_order();
    //    }
    //    {
    //        use stack::Stack;
    //        #[derive(PartialEq, Eq, Debug)]
    //        struct TestStruct {
    //            a: i32,
    //        }
    //
    //        let a = TestStruct { a: 5 };
    //        let b = TestStruct { a: 9 };
    //
    //        let mut s = Stack::<&TestStruct>::new();
    //        assert_eq!(s.pop(), None);
    //
    //        s.push(&a);
    //        s.push(&b);
    //        println!("{:?}", s);
    //
    //        assert_eq!(s.pop(), Some(&b));
    //        assert_eq!(s.pop(), Some(&a));
    //        assert_eq!(s.pop(), None);
    //    }
    //    {
    //        use queue::*;
    //
    //        let mut q = Queue::new();
    //        q.push(1);
    //        q.push(2);
    //        println!("{:?}", q);
    //        q.pop();
    //        println!("{:?}", q);
    //        q.pop();
    //    }
    //    {
    //        use pq::*;
    //
    //        let mut btree = BinaryHeap::with_capacity(10);
    //        btree.push(5);
    //        btree.push(4);
    //        btree.push(3);
    //        btree.push(2);
    //        btree.push(1);
    //        println!("{:?}", btree);
    //    }
}
