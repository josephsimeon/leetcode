use leetcode_2::{ListNode, add_two_numbers};

fn main() {
    let mut l1 = ListNode::new(2);
    let mut l2 = ListNode::new(4);
    let l3 = ListNode::new(3);

    l2.next = Some(Box::new(l3));
    l1.next = Some(Box::new(l2));

    let mut l4 = ListNode::new(5);
    let mut l5 = ListNode::new(6);
    let l6 = ListNode::new(4);

    l5.next = Some(Box::new(l6));
    l4.next = Some(Box::new(l5));

    println!("{:?}", add_two_numbers(Some(Box::new(l1)), Some(Box::new(l4))));
}
