// fn add_two_nums(l1: Vec<u32>, l2: Vec<u32>) -> Vec<u32> {
//     let mut hold_num: i64 = 0;

//     for i in 0..l1.len() {
//         hold_num += l1[i] as i64 * 10_i64.pow(i as u32);
//         println!("L1 Index {} is {}", i, l1[i]);
//     }

//     for i in 0..l2.len() {
//         hold_num += l2[i] as i64 * 10_i64.pow(i as u32);
//         println!("L2 Index {} is {}", i, l2[i]);
//     }

//     hold_num
//         .to_string()
//         .chars()
//         .rev()
//         .map(|c| c.to_digit(10).unwrap())
//         .collect()
// }

// fn main() {
//     let l1 = vec![2, 4, 3];
//     let l2 = vec![5, 6, 4];
//     let result = add_two_nums(l1, l2);
//     println!(" Main Result: {result:?}");
// }

// Definition for singly-linked list.
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
    let mut l1 = l1;
    let mut l2 = l2;
    let mut carry = 0;
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let mut sum = carry;

        if let Some(node) = l1 {
            sum += node.val;
            l1 = node.next;
        }

        if let Some(node) = l2 {
            sum += node.val;
            l2 = node.next;
        }

        carry = sum / 10;
        tail.next = Some(Box::new(ListNode::new(sum % 10)));
        tail = tail.next.as_mut().unwrap();
    }

    head.next
}

fn main() {
    let l1 = Some(Box::new(ListNode::new(342)));
    let l2 = Some(Box::new(ListNode::new(465)));
    let result = add_two_numbers(l1, l2);
    println!(" Main Result: {result:?}");
}
