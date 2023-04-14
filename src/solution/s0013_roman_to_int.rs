#[derive(Debug, Clone, Copy)]
enum Roman {
    ZERO = 0,
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}
impl From<char> for Roman {
    fn from(c: char) -> Self {
        match c {
            'I' => Roman::I,
            'V' => Roman::V,
            'X' => Roman::X,
            'L' => Roman::L,
            'C' => Roman::C,
            'D' => Roman::D,
            'M' => Roman::M,
            _ => Roman::ZERO,
        }
    }
}
pub struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        if s.len() < 1 {
            return 0;
        }

        let previous: Roman = Roman::from(s.chars().nth(0).unwrap());
        let mut previous_value = previous as i32;
        let mut result: i32 = previous_value;
        for i in 1..s.len() {
            let char = s.chars().nth(i).unwrap();
            let roman: Roman = Roman::from(char);
            let value: i32 = roman as i32;
            if previous_value < value {
                result += value - 2 * previous_value;
            } else {
                result += value;
            }
            previous_value = value;
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let roman_str = String::from("MCMXCIV");
        assert_eq!(Solution::roman_to_int(roman_str), 1994);
    }
}
