pub struct Solution {}

impl Solution {
    // TODO: Make this better
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut len = nums.len();
        if len < 2 {
            return len as i32;
        }
        let mut i: usize = 0;
        let mut j: usize = i + 1;
        loop {
            if len == 0 || i > len - 2 || j > len - 1 {
                break;
            }

            let num = nums[i];
            loop {
                if j > len - 1 {
                    break;
                }
                if num == nums[j] {
                    nums.remove(j);
                    len -= 1;
                } else {
                    j += 1;
                }
            }

            i += 1;
            j = i + 1;
        }

        return len as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let mut nums: Vec<i32> = vec![1, 1, 2];
        let expected_nums: Vec<i32> = vec![1, 2];

        let k = Solution::remove_duplicates(&mut nums); // Calls your implementation

        assert_eq!(nums.len() as i32, k);

        for i in 0..nums.len() {
            assert_eq!(expected_nums[i], nums[i]);
        }
    }
    #[test]
    fn example_2() {
        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let expected_nums: Vec<i32> = vec![0, 1, 2, 3, 4];

        let k = Solution::remove_duplicates(&mut nums); // Calls your implementation

        assert_eq!(nums.len() as i32, k);

        for i in 0..nums.len() {
            println!("{}={}", nums[i], expected_nums[i]);
            assert_eq!(expected_nums[i], nums[i]);
        }
    }
}
