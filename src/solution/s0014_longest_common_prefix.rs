pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut i = 0;
        let mut result_string: String = "".to_owned();
        'outer: loop {
            // i : over characters
            if strs[0].len() < i + 1 {
                break 'outer;
            }

            let common = strs[0].chars().nth(i).unwrap();
            for j in 1..strs.len() {
                // j : over strings for this character
                if strs[j].len() < i + 1 {
                    break 'outer;
                }

                let curr_string = strs.get(j).unwrap();
                let c = curr_string.chars().nth(i).unwrap();
                if c != common {
                    break 'outer;
                }
            }
            i += 1;
            result_string.push(common);
        }
        return result_string;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn empty_output() {
        let strs = [
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];
        let output = Solution::longest_common_prefix(Vec::from(strs));
        assert_eq!(output, "");
    }

    #[test]
    fn it_works() {
        let strs = [
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        let output = Solution::longest_common_prefix(Vec::from(strs));
        assert_eq!(output, "fl");
    }
}
