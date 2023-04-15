pub struct Solution {}

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_chars: Vec<char> = haystack.chars().collect();
        let needle_chars: Vec<char> = needle.chars().collect();
        let mut j = 0;
        let mut i: usize = 0;
        while i < haystack_chars.len() {
            dbg!(i);
            if dbg!(j) >= dbg!(needle_chars.len()) {
                return (i - needle_chars.len()) as i32;
            }

            if needle_chars.get(j).unwrap() == haystack_chars.get(i).unwrap() {
                j = j + 1;
                i = i + 1;
            } else {
                i = i - j + 1;
                j = 0;
            }
        } 
        if j >= needle_chars.len() {
            return (haystack_chars.len() - needle_chars.len()) as i32;
        } else {
            return -1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1(){
        let result = Solution::str_str("sadbutsad".to_owned(), "sad".to_owned());

        assert_eq!(result, 0);
    }
    #[test]
    fn example_2(){
        let result = Solution::str_str("leetcode".to_owned(), "leeto".to_owned());

        assert_eq!(result, -1);
    }

    #[test]
    fn single_char() {
        let result = Solution::str_str("a".to_owned(), "a".to_owned());

        assert_eq!(result, 0);
    }

    #[test]
    fn empty() {
        let result = Solution::str_str("".to_owned(), "anything".to_owned());

        assert_eq!(result, -1);
    }
    #[test]
    fn mississippi() {
        let result = Solution::str_str("mississippi".to_owned(), "issip".to_owned());

        assert_eq!(result, 4);
    }
}
