pub struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry = -1;
        let mut result: Vec<i32> = digits.iter().rev().map(|n| {
            let r: i32;
            if carry == -1 {
                r = *n + 1;
            } else {
                r = *n + carry;
            }
            carry = r / 10;
            return r % 10;
        }).collect();

        if carry > 0 {
            result.push(carry);
        }

        result.reverse();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1(){
        let digits = vec![1,2,3];
        let result = Solution::plus_one(digits);

        assert_eq!(result, [1,2,4]);
    }
    #[test]
    fn example_2(){
        let digits = vec![4,3,2,1];
        let result = Solution::plus_one(digits);

        assert_eq!(result, [4,3,2,2]);
    }
    #[test]
    fn example_3(){
        let digits = vec![9];
        let result = Solution::plus_one(digits);

        assert_eq!(result, [1,0]);
    }
    #[test]
    fn example_4(){
        let digits = vec![9,9,9];
        let result = Solution::plus_one(digits);

        assert_eq!(result, [1,0,0,0]);
    }
}
