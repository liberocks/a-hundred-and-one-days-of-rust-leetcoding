// Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/description/

// Given a string s, find the length of the longest
// substring without repeating characters.

// Example 1:

// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
// Example 2:

// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:

// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

// Constraints:

// 0 <= s.length <= 5 * 104
// s consists of English letters, digits, symbols and spaces.

use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut left = 0;
    let mut right = 0;
    let mut unique_letters = HashSet::new();
    let mut longest_substring = 0;

    for letter in s.chars() {
        if unique_letters.insert(letter) {
            longest_substring = longest_substring.max(right + 1 - left);
        } else {
            while let Some(ch) = s.chars().nth(left) {
                unique_letters.remove(&ch);
                left += 1;
                if ch == letter {
                    break;
                }
            }
            unique_letters.insert(letter);
        }

        right += 1;
    }

    longest_substring as i32
}

#[cfg(test)]
mod day_006 {

    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
        assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
        assert_eq!(length_of_longest_substring(" ".to_string()), 1);
    }
}
