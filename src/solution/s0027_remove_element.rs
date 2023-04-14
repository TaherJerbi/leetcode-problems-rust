pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|n| *n != val);
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut numbers = vec![1,2,3];
        assert_eq!(Solution::remove_element(&mut numbers, 3),2);
        assert_eq!(numbers.len(), 2);
    }
    #[test]
    fn remove_all() {
        let mut numbers = vec![3,3,3];
        assert_eq!(Solution::remove_element(&mut numbers, 3),0);
        assert_eq!(numbers.len(), 0);
    }
}
