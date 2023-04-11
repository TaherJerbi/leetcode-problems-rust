use std::fmt::Display;

struct Solution {}
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
  }
  fn link(&mut self, val: i32) {
    self.next = Some(Box::new(ListNode::new(val)));
  }

  fn from(numbers: &[i32]) -> Option<Self> {
    if numbers.len() == 0 { return None } 

    let mut root = ListNode::new(*numbers.get(0).unwrap()); 
    let mut curr = &mut root;

    for i in 1..numbers.len() {
      curr.link(*numbers.get(i).unwrap());
      let next = curr.next.as_mut().unwrap();
      curr = next;
    }

    return Some(root);
  }
}

impl PartialEq<Vec<i32>> for ListNode {
    fn eq(&self, other: &Vec<i32>) -> bool {
        let mut curr = self;
        other.iter().all(|n| {
          let equals = curr.val == *n;
          let next = curr.next.as_ref();
          match next {
            Some(l) => {
              curr = l;
            }
            None => ()
          }
          return equals;
        })
    }
}
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return Solution::add_two_numbers_rec(l1, l2, 0);
    }

    pub fn add_two_numbers_rec(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {

      if let Some(list1) = l1 {
        if let Some(list2) = l2 {
          // list1 , list2 = SOME
          let n = list1.val + list2.val + carry;

          let result = n % 10;
          let carry = n / 10;

          return Some(Box::new(ListNode { val: result, next: Solution::add_two_numbers_rec(list1.next, list2.next, carry) }))

        } else {
          // list1 = SOME, list2 = NONE
          let n = list1.val + carry;
          let result = n % 10;
          let carry = n / 10;

          return Some(Box::new(ListNode { val: result, next: Solution::add_two_numbers_rec(list1.next, None,carry) }))
        }
      } else if let Some(list2) = l2 {
        // list1 = NONE, list2 = SOME
        let n = list2.val + carry;
          let result = n % 10;
          let carry = n / 10;

          return Some(Box::new(ListNode { val: result, next: Solution::add_two_numbers_rec(list2.next, None,carry) }))
        
    } else {
      // list1, list2 = NONE
      if carry > 0 {
        return Some(Box::new(ListNode::new(carry)));
      } else {
        None
      }
    }
  }
}

#[cfg(test)]
mod tests {

    use super::{Solution, ListNode};

    #[test]
    fn equality() {
        assert_eq!(ListNode::new(2),ListNode::new(2)); 
        assert_ne!(ListNode::new(2),ListNode::new(3)); 
    }

    #[test]
    fn from_array() {
        let numbers = vec![1,2,3];
        let list = ListNode::from(&numbers).unwrap();
        assert_eq!(list.val,1);
        let next = list.next.unwrap();
        assert_eq!(next.val, 2);
    }

    #[test]
    fn eq_array() {
      let list = ListNode::from(&vec![1,2,3]);
      let numbers = vec![1,2,3];

      assert!(list.unwrap().eq(&numbers));
    }
    #[test]
    fn example_1(){
      let n1 = vec![1,2,3];
      let n2 = vec![2,3,4];
      let result = Solution::add_two_numbers(
        Some(Box::new(ListNode::from(&n1).unwrap())),
        Some(Box::new(ListNode::from(&n2).unwrap())),
        );
      
      
      assert!(dbg!(result).unwrap().as_ref().eq(&vec![3,5,7]));
    }

    #[test]
    fn example_2() {
      let n1 = vec![1,2,9];
      let n2 = vec![2,3,4];
      let result = Solution::add_two_numbers(
        Some(Box::new(ListNode::from(&n1).unwrap())),
        Some(Box::new(ListNode::from(&n2).unwrap())),
        );
      
      
      assert!(dbg!(result).unwrap().as_ref().eq(&vec![3,5,3,1]));
    }

    #[test]
    fn example_3() {
      let n1 = vec![1];
      let n2 = vec![9,9,9];
      let result = Solution::add_two_numbers(
        Some(Box::new(ListNode::from(&n1).unwrap())),
        Some(Box::new(ListNode::from(&n2).unwrap())),
        );
      
      
      assert!(dbg!(result).unwrap().as_ref().eq(&vec![0,0,0,1]));
    }
}
