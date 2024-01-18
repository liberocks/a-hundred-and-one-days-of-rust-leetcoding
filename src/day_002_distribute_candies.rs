// Distribute Candies
// https://leetcode.com/problems/distribute-candies/
//
// Alice has n candies, where the ith candy is of type candyType[i]. Alice noticed that she started to gain weight, so she visited a doctor.
//
// The doctor advised Alice to only eat n / 2 of the candies she has (n is always even). Alice likes her candies very much, and she wants to eat the maximum number of different types of candies while still following the doctor's advice.
//
// Given the integer array candyType of length n, return the maximum number of different types of candies she can eat if she only eats n / 2 of them.
//
// Example 1:
//
// Input: candyType = [1,1,2,2,3,3]
// Output: 3
// Explanation: Alice can only eat 6 / 2 = 3 candies. Since there are only 3 types, she can eat one of each type.
// Example 2:
//
// Input: candyType = [1,1,2,3]
// Output: 2
// Explanation: Alice can only eat 4 / 2 = 2 candies. Whether she eats types [1,2], [1,3], or [2,3], she still can only eat 2 different types.
// Example 3:
//
// Input: candyType = [6,6,6,6]
// Output: 1
// Explanation: Alice can only eat 4 / 2 = 2 candies. Even though she can eat 2 candies, she only has 1 type.
//
// Constraints:
//
// n == candyType.length
// 2 <= n <= 104
// n is even.
// -105 <= candyType[i] <= 105

use std::collections::HashSet;

pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    let mut unique_candies: HashSet<i32> = HashSet::new();
    let half_total_candies: usize = candy_type.len() / 2;

    for candy in candy_type.into_iter() {
        if unique_candies.len() >= half_total_candies {
            break;
        }

        unique_candies.insert(candy);
    }

    unique_candies.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute_candies() {
        assert_eq!(distribute_candies(vec![1, 1, 2, 2, 3, 3]), 3);
        assert_eq!(distribute_candies(vec![1, 1, 2, 3]), 2);
        assert_eq!(distribute_candies(vec![6, 6, 6, 6]), 1);
    }
}
