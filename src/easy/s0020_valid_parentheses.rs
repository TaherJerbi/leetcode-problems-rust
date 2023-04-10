pub struct Solution {}

fn opening_parenthesis(c: char) -> char {
    match c {
        ')' => '(',
        '}' => '{',
        ']' => '[',
        _ => panic!(),
    }
}
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());

        for c in s.chars() {
            match c {
                '(' | '{' | '[' => {
                    stack.push(c);
                }
                ')' | '}' | ']' => {
                    let head = stack.pop();
                    let opening = opening_parenthesis(c);
                    match head {
                        Some(char) => {
                            if char != opening {
                                return false;
                            }
                        }
                        _ => return false,
                    }
                }
                _ => (),
            }
        }

        // stack is empty
        return stack.len() == 0;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn is_valid() {
        assert_eq!(Solution::is_valid("()".to_owned()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_owned()), true);
        assert_eq!(Solution::is_valid("{([])}".to_owned()), true);
    }
    #[test]
    fn is_invalid() {
        assert_eq!(Solution::is_valid("(]".to_owned()), false);
        assert_eq!(Solution::is_valid("()[)]".to_owned()), false);
        assert_eq!(Solution::is_valid("( [ ) ] ".to_owned()), false);
    }
}
