use super::s0002_add_two_numbers::ListNode;


pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(list)) | (Some(list), None) => {
                Some(list)
            },
            (Some(list1), Some(list2)) => {
                if list1.val < list2.val {
                    let head = ListNode { val: list1.val, next:  Solution::merge_two_lists(list1.next, Some(list2)) };

                    Some(Box::new(head))
                } else {
                    let head = ListNode {val: list2.val, next:  Solution::merge_two_lists(Some(list1), list2.next) };
                    
                    Some(Box::new(head))
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::solution::s0002_add_two_numbers::ListNode;

    use super::Solution;

    #[test]
    fn it_works() {
        let list1 = ListNode::from(&vec![2,5,6]).unwrap();
        let list2 = ListNode::from(&vec![1,10,20]).unwrap();

        let merged_list = Solution::merge_two_lists(Some(Box::new(list1)), Some(Box::new(list2)));

        assert!(merged_list.unwrap().as_ref().eq(&vec![1,2,5,6,10,20]));
    }

    #[test]
    fn one_none(){
        let list = ListNode::from(&vec![1,10,20]).unwrap();

        let merged_list = Solution::merge_two_lists(Some(Box::new(list)), None);

        assert!(merged_list.unwrap().as_ref().eq(&vec![1,10,20]));
    }
    #[test]
    fn two_none() {

        let merged_list = Solution::merge_two_lists(None, None);

        assert!(merged_list.is_none());
    }
}
