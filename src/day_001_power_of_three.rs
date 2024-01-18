// Power of Three
// https://leetcode.com/problems/power-of-three/description/

// Given an integer n, return true if it is a power of three. Otherwise, return false.

// An integer n is a power of three, if there exists an integer x such that n == 3x.

// Example 1:

// Input: n = 27
// Output: true
// Explanation: 27 = 33
// Example 2:

// Input: n = 0
// Output: false
// Explanation: There is no x where 3x = 0.
// Example 3:

// Input: n = -1
// Output: false
// Explanation: There is no x where 3x = (-1).

// Constraints:

// -231 <= n <= 231 - 1

// Follow up: Could you solve it without loops/recursion?

pub fn is_power_of_three(n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    let mut nmut: i32 = n;

    while nmut % 3 == 0 {
        nmut /= 3;
    }

    nmut == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_power_of_three() {
        assert_eq!(is_power_of_three(81), true);
        assert_eq!(is_power_of_three(45), false);
        assert_eq!(is_power_of_three(27), true);
        assert_eq!(is_power_of_three(0), false);
        assert_eq!(is_power_of_three(-1), false);
    }
}
