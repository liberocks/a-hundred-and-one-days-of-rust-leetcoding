# A Hundred and One Days of Rust Leetcoding

This project contains solutions to Leetcode problems implemented in Rust. Each day, a new problem is solved and added to the corresponding module.

![Unit test](https://github.com/liberocks/a-hundred-and-one-days-of-rust-leetcoding/actions/workflows/test.yaml/badge.svg)

## Modules

- `day_000_palindrome_number`: Solution for the Palindrome Number problem.
- `day_001_power_of_three`: Solution for the Power of Three problem.
- `day_002_distribute_candies`: Solution for the Distribute Candies problem.
- `day_003_gray_code`: Solution for the Gray Code problem.
- `day_004_valid_parentheses`: Solution for the Valid Parentheses problem.
- `day_005_combination_sum`: Solution for the Combination Sum problem.

## Running the Project

Before running the project, you need to have Rust installed. You can install Rust by following the instructions [here](https://www.rust-lang.org/tools/install). Once Rust is installed, you can run the project by executing the following command to run all problems:

```bash
cargo test
```

If you want to run specific problems, you can execute the following command : `cargo test day_XXX`. Change XXX with the day number you want to run. For example, if you want to run the day zero, you can execute the following command:

```bash
cargo test day_000
```