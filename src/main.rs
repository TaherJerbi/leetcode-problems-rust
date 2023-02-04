use crate::easy::roman_to_int;
pub mod easy;
fn main() {
    let roman = String::from("IX");

    let result = roman_to_int::Solution::roman_to_int(roman.clone());
    println!("{} = {}", roman, result);
}
