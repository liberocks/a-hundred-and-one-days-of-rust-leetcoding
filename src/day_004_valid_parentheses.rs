// Valid Parentheses
// https://leetcode.com/problems/valid-parentheses/description/

// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:

// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.

// Example 1:

// Input: s = "()"
// Output: true
// Example 2:

// Input: s = "()[]{}"
// Output: true
// Example 3:

// Input: s = "(]"
// Output: false

// Constraints:

// 1 <= s.length <= 104
// s consists of parentheses only '()[]{}'.

pub fn is_valid(s: String) -> bool {
    let mut smut: String = s;

    while smut.len() > 0 {
        if let Some(index) = smut.find("()") {
            smut.remove(index + 1);
            smut.remove(index);
        } else if let Some(index) = smut.find("[]") {
            smut.remove(index + 1);
            smut.remove(index);
        } else if let Some(index) = smut.find("{}") {
            smut.remove(index + 1);
            smut.remove(index);
        } else {
            break;
        }
    }

    smut.len() == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid("()".to_string()), true);
        assert_eq!(is_valid("()[]{}".to_string()), true);
        assert_eq!(is_valid("(]".to_string()), false);
    }
}
