// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ln = Box::new(ListNode::new(0));
        let mut p_ln = ln.as_mut();

        let mut l1 = l1;
        let mut l2 = l2;

        let mut p_l1 = &mut l1;
        let mut p_l2 = &mut l2;

        let mut carry: i32 = 0;
        loop {
            let mut sum = carry;

            if let Some(n1) = p_l1 {
                sum += n1.val;
                p_l1 = &mut n1.next;
            }

            if let Some(n2) = p_l2 {
                sum += n2.val;
                p_l2 = &mut n2.next;
            }

            carry = sum / 10;
            p_ln.val = sum % 10;

            if p_l1.is_none() & p_l2.is_none() {
                if carry > 0 {
                    p_ln.next = Some(Box::new(ListNode::new(1)));
                }
                break;
            }

            p_ln.next = Some(Box::new(ListNode::new(0)));
            p_ln = p_ln.next.as_mut().unwrap();

        }

        return Some(ln);
    }
}