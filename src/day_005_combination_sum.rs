// Combination Sum
// https://leetcode.com/problems/combination-sum/

// Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.

// The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the
// frequency
//  of at least one of the chosen numbers is different.

// The test cases are generated such that the number of unique combinations that sum up to target is less than 150 combinations for the given input.

// Example 1:

// Input: candidates = [2,3,6,7], target = 7
// Output: [[2,2,3],[7]]
// Explanation:
// 2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
// 7 is a candidate, and 7 = 7.
// These are the only two combinations.
// Example 2:

// Input: candidates = [2,3,5], target = 8
// Output: [[2,2,2,2],[2,3,3],[3,5]]
// Example 3:

// Input: candidates = [2], target = 1
// Output: []

// Constraints:

// 1 <= candidates.length <= 30
// 2 <= candidates[i] <= 40
// All elements of candidates are distinct.
// 1 <= target <= 40

pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut current: Vec<i32> = Vec::new();

    recursive_sum(&candidates, target, 0, &mut current, &mut result);

    result
}

fn recursive_sum(
    candidates: &Vec<i32>,
    target: i32,
    start: usize,
    current: &mut Vec<i32>,
    result: &mut Vec<Vec<i32>>,
) {
    if target < 0 {
        return;
    }
    if target == 0 {
        result.push(current.clone());
        return;
    }
    for i in start..candidates.len() {
        current.push(candidates[i]);
        recursive_sum(candidates, target - candidates[i], i, current, result);
        current.pop();
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_combination_sum() {
        assert_eq!(
            combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![2, 2, 3], vec![7]]
        );
        assert_eq!(
            combination_sum(vec![2, 3, 5], 8),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
        assert_eq!(combination_sum(vec![2], 1), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_recursive_sum() {
        {
            let mut result: Vec<Vec<i32>> = Vec::new();
            let mut current: Vec<i32> = Vec::new();
            let candidates: Vec<i32> = vec![2, 3, 6, 7];
            let target: i32 = 7;
            recursive_sum(&candidates, target, 0, &mut current, &mut result);
            assert_eq!(result, vec![vec![2, 2, 3], vec![7]]);
        }

        {
            let mut result: Vec<Vec<i32>> = Vec::new();
            let mut current: Vec<i32> = Vec::new();
            let candidates: Vec<i32> = vec![2, 3, 5];
            let target: i32 = 8;
            recursive_sum(&candidates, target, 0, &mut current, &mut result);
            assert_eq!(result, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]);
        }
    }
}
