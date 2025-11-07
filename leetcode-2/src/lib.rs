#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

const CARRY_THRESHOLD: i32 = 10;
const CARRY_VALUE: i32 = 1;

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut vector: Vec<i32> = Vec::new();

    let mut head1 = l1;
    let mut head2 = l2;

    let mut end_of_head1: bool = false;
    let mut end_of_head2: bool = false;

    'linked_list: loop {
        let val_of_head1 = if end_of_head1 { 0 } else { head1.clone().unwrap().val };
        let val_of_head2 = if end_of_head2 { 0 } else { head2.clone().unwrap().val };

        vector.push(val_of_head1 + val_of_head2);

        let check_for_end_of_head1 = if !end_of_head1 { head1.clone().unwrap().next == None } else { true };
        if check_for_end_of_head1 && !end_of_head1 {
            end_of_head1 = true;
        }

        let check_for_end_of_head2 = if !end_of_head2 { head2.clone().unwrap().next == None } else { true };
        if check_for_end_of_head2 && !end_of_head2 {
            end_of_head2 = true;
        }

        let check_for_none = check_for_end_of_head1 && check_for_end_of_head2;
        if check_for_none {
            break 'linked_list;
        }

        if !end_of_head1 {
            head1 = head1.clone().unwrap().next;
        }

        if !end_of_head2 {
            head2 = head2.clone().unwrap().next;
        }
    }

    let mut carry = 0;
    for i in 0..vector.len() {
        vector[i] += carry;

        carry = 0;
        if vector[i] >= CARRY_THRESHOLD {
            vector[i] -= CARRY_THRESHOLD;
            carry = CARRY_VALUE;
        }
    }

    if carry == CARRY_VALUE { vector.push(carry) };

    let limit = vector.len() - 1;
    let mut result = ListNode::new(vector[limit]);
    for i in (0..limit).rev() {
        let mut list = ListNode::new(vector[i]);
        list.next = Some(Box::new(result));
        result = list.clone();
    }

    Some(Box::new(result))
}

