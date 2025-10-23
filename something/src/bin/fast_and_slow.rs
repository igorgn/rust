/// Given a singly linked list, determine if it contains a cycle.
/// Return `true` if there is a cycle, otherwise `false`.
///
/// Example:
/// Input: 1 -> 2 -> 3 -> 4 -> 2 (cycle back to node with value 2)
/// Output: true

fn main() {}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn middle_node(head: Option<Box<ListNode>>) -> i32 {
    let mut slow = &head;
    let mut fast = &head;

    while let (Some(slow_node), Some(fast_node)) =  (slow, fast) {
        slow = &slow_node.next;
        fast = match &fast_node.next {
            Some(node) => &node.next,
            None => &None,
        };
    }
    slow.as_ref().unwrap().val
}

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
fn test_odd_length() {
    let mut n1 = Box::new(ListNode::new(1));
    let mut n2 = Box::new(ListNode::new(2));
    let mut n3 = Box::new(ListNode::new(3));
    let mut n4 = Box::new(ListNode::new(4));
    let n5 = Box::new(ListNode::new(5));
    n4.next = Some(n5);
    n3.next = Some(n4);
    n2.next = Some(n3);
    n1.next = Some(n2);
    assert_eq!(middle_node(Some(n1)), 3);
}

#[test]
fn test_even_length() {
    let mut n1 = Box::new(ListNode::new(1));
    let mut n2 = Box::new(ListNode::new(2));
    let mut n3 = Box::new(ListNode::new(3));
    let mut n4 = Box::new(ListNode::new(4));
    let mut n5 = Box::new(ListNode::new(5));
    let n6 = Box::new(ListNode::new(6));
    n5.next = Some(n6);
    n4.next = Some(n5);
    n3.next = Some(n4);
    n2.next = Some(n3);
    n1.next = Some(n2);
    assert_eq!(middle_node(Some(n1)), 4);
}

}