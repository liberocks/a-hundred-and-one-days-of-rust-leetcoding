// Palindrome Number
// https://leetcode.com/problems/palindrome-number/description/

// Given an integer x, return true if x is a palindrome and false otherwise.

// Example 1:
// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.
// Example 2:

// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
// Example 3:

// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

// Constraints:

// -231 <= x <= 231 - 1

// Follow up: Could you solve it without converting the integer to a string?

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let mut xmut: i32 = x;
    let mut reversed: i32 = 0;

    while xmut > 0 {
        let digit: i32 = xmut % 10;
        reversed = reversed * 10 + digit;
        xmut /= 10;
    }

    reversed == x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
    }
}
