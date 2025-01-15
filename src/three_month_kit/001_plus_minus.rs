//! # Plus Minus
//!
//! ## Problem Statement
//!
//! Given an array of integers, calculate the ratios of its elements that are positive, negative, and zero. Print the decimal value of each fraction on a new line with  places after the decimal.
//!
//! > **Note:** This challenge introduces precision problems. The test cases are scaled to six decimal places, though answers with absolute error of up to  are acceptable.
//!
//! ### Example
//!
//! ```ignore
//! arr = [1,1,0,-1,-1]
//! ```
//!
//! There are `n = 5` elements, two positive, two negative and one zero.
//! Their ratios are `2/5 = 0.400000`, `2/5 = 0.400000` and `1/5 = 0.200000`. Results are printed as:
//!
//! ```ignore
//! 0.400000
//! 0.400000
//! 0.200000
//! ```
//!
//! ### Function Description
//!
//! `plusMinus` has the following parameter(s):
//!
//! - int arr[n]: an array of integers
//!
//! #### Print
//!
//! Print the ratios of positive, negative and zero values in the array.
//! Each value should be printed on a separate line with `6` digits after the decimal.
//! The function should not return a value.
//!
//! #### Input Format
//!
//! The first line contains an integer, `n`, the size of the array.
//! The second line contains  space-separated integers that describe `arr[n]`.
//!
//! #### Constraints
//!
//! - `0 < n <= 100`
//! - `-100 <= arr[i] <= 100`
//!
//! #### Output Format
//!
//! Print the following 3 lines, each to 6 decimals:
//!
//! 1. proportion of positive values
//! 1. proportion of negative values
//! 1. proportion of zeros
//!
//! ### Sample Input
//!
//! ```ignore
//! STDIN           Function
//! -----           --------
//! 6               arr[] size n = 6
//! -4 3 -9 0 4 1   arr = [-4, 3, -9, 0, 4, 1]
//! ```
//!
//! ### Sample Output
//!
//! ```ignore
//! 0.500000
//! 0.333333
//! 0.166667
//! ```
//!
//! ### Explanation
//!
//! There are 3 positive numbers, 2 negative numbers, and 1 zero in the array.
//! The proportions of occurrence are positive: `3/6 = 0.500000`, negative: `2/6 = 0.333333` and zeros: `3/6 = 0.166667`.
//!
//! ## Solution
//!
//! The below code contains my solution, ignoring the supplied `main` from HR, instead utilizing
//! `rstest` & test cases to execute the code. All test cases are straight from HR, no special edge
//! cases were needed to pass all tests, including hidden ones.
//!
//! This solution works with a simple single pass along the given slice of signed integers,
//! incrementing an appropriate counter depending on if the value currently being compared is
//! greater than, less than, or equal to zero. It utilizes built-in iteration tools to do so,
//! `fold`ing, the slice into a tuple of counters using a supplied increment function.

use std::cmp::Ordering;

use rstest::rstest;

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

/// gets ratio strings using helper & prints them
fn plusMinus(arr: &[i32]) {
    let (p, n, z) = get_ratios(arr);
    print!("{p}\n{n}\n{z}");
}

/// Calculates ratios by counting occurrences of positive, negative, & zero, then dividing by
/// length. Finally, get_ratios formats the results as strings w/ 6 decimal places. This formatting
/// approach is naive and could lead to incorrect output due to behaviour of floats, but no such
/// errors occurred in HR's test cases, so further complication to guard against float rounding
/// errors was unnecessary.
fn get_ratios(arr: &[i32]) -> (String, String, String) {
    let (pos, neg, zer) = count_pnz(arr);
    let len = arr.len();

    let p = pos as f32 / len as f32;
    let n = neg as f32 / len as f32;
    let z = zer as f32 / len as f32;

    (format!("{p:.6}"), format!("{n:.6}"), format!("{z:.6}"))
}

/* FIXME: remove inline tests for submission as hr env won't have rstest installed
 */
#[rstest]
#[case(vec![-4, 3, -9, 0, 4, 1], ("0.500000", "0.333333", "0.166667"))]
#[case(vec![1, 2, 3, -1, -2, -3, 0, 0], ("0.375000", "0.375000", "0.250000"))]
fn test_get_ratios(#[case] input: Vec<i32>, #[case] output: (&str, &str, &str)) {
    assert_eq!(
        get_ratios(&input),
        (
            output.0.to_string(),
            output.1.to_string(),
            output.2.to_string(),
        )
    );
}

/// Wrapper on fold to count each occurence in a given slice.
fn count_pnz(arr: &[i32]) -> (u8, u8, u8) {
    arr.iter().fold((0, 0, 0), inc_count_pnz)
}

/* FIXME: remove inline tests for submission as hr env won't have rstest installed
 */
#[rstest]
#[case(vec![-4, 3, -9, 0, 4, 1], (3, 2, 1))]
fn test_count_pnz(#[case] input: Vec<i32>, #[case] output: (u8, u8, u8)) {
    assert_eq!(count_pnz(&input), output);
}

/// Increment the appropriate counter based on the ordering compared to zero.
fn inc_count_pnz(prev: (u8, u8, u8), val: &i32) -> (u8, u8, u8) {
    match val.cmp(&0) {
        Ordering::Greater => (prev.0 + 1, prev.1, prev.2),
        Ordering::Less => (prev.0, prev.1 + 1, prev.2),
        Ordering::Equal => (prev.0, prev.1, prev.2 + 1),
    }
}

/* FIXME: remove inline tests for submission as hr env won't have rstest installed
 */
#[rstest]
#[case(((0, 0, 0), -4), (0, 1, 0))]
#[case(((0, 1, 0), 3), (1, 1, 0))]
#[case(((1, 1, 0), -9), (1, 2, 0))]
#[case(((1, 2, 0), 0), (1, 2, 1))]
#[case(((1, 2, 1), 4), (2, 2, 1))]
#[case(((2, 2, 1), 1), (3, 2, 1))]
fn test_inc_count_pnz(#[case] input: ((u8, u8, u8), i32), #[case] output: (u8, u8, u8)) {
    let (p, v) = input;
    assert_eq!(inc_count_pnz(p, &v), output);
}
