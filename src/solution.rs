#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1, l2);
    let (mut lhs_done, mut rhs_done) = (false, false);
    let mut carry = 0;
    let mut current = None;

    loop {
        let lhs = match l1 {
            Some(node) => {
                l1 = node.next;
                node.val
            }
            None => {
                lhs_done = true;
                0
            }
        };

        let rhs = match l2 {
            Some(node) => {
                l2 = node.next;
                node.val
            }
            None => {
                rhs_done = true;
                0
            }
        };

        if lhs_done && rhs_done && carry == 0 {
            break;
        }

        let mut digit = lhs + rhs + carry;

        carry = if digit >= 10 {
            digit -= 10;
            1
        } else {
            0
        };

        let mut node = ListNode::new(digit);
        node.next = current;
        current = Some(Box::new(node));
    }

    reverse(current)
}

/// Reverse a 'leet-code' list
///
fn reverse(list: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = list;
    let mut tail = None;

    while let Some(mut current) = head.take() {
        head = current.next;
        current.next = tail;
        tail = Some(current);
    }
    tail
}

pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &x in vec.iter().rev() {
        let mut node = ListNode::new(x);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}
