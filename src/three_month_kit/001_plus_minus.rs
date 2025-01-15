//! # Plus Minus
//!
//! Given an array of integers, calculate the ratios of its elements that are positive, negative, and zero. Print the decimal value of each fraction on a new line with  places after the decimal.
//!
//! > **Note:** This challenge introduces precision problems. The test cases are scaled to six decimal places, though answers with absolute error of up to  are acceptable.
//!
//! ## Example
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
//! ## Function Description
//!
//! `plusMinus` has the following parameter(s):
//!
//! - int arr[n]: an array of integers
//!
//! ### Print
//!
//! Print the ratios of positive, negative and zero values in the array.
//! Each value should be printed on a separate line with `6` digits after the decimal.
//! The function should not return a value.
//!
//! ### Input Format
//!
//! The first line contains an integer, `n`, the size of the array.
//! The second line contains  space-separated integers that describe `arr[n]`.
//!
//! ### Constraints
//!
//! - `0 < n <= 100`
//! - `-100 <= arr[i] <= 100`
//!
//! ### Output Format
//!
//! Print the following 3 lines, each to 6 decimals:
//!
//! 1. proportion of positive values
//! 1. proportion of negative values
//! 1. proportion of zeros
//!
//! ## Sample Input
//!
//! ```ignore
//! STDIN           Function
//! -----           --------
//! 6               arr[] size n = 6
//! -4 3 -9 0 4 1   arr = [-4, 3, -9, 0, 4, 1]
//! ```
//!
//! ## Sample Output
//!
//! ```ignore
//! 0.500000
//! 0.333333
//! 0.166667
//! ```
//!
//! ## Explanation
//!
//! There are 3 positive numbers, 2 negative numbers, and 1 zero in the array.
//! The proportions of occurrence are positive: `3/6 = 0.500000`, negative: `2/6 = 0.333333` and zeros: `3/6 = 0.166667`.

use std::cmp::Ordering;

use rstest::rstest;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let (p, n, z) = get_ratios(arr);
    println!("{p}");
    println!("{n}");
    println!("{z}");
}

fn get_ratios(arr: &[i32]) -> (Decimal, Decimal, Decimal) {
    let (pos, neg, zer) = count_pnz(arr);
    let len = arr.len();

    let p = pos as f32 / len as f32;
    println!("pos {pos} / {len} => {p}");
    let n = neg as f32 / len as f32;
    println!("neg {neg} / {len} => {n}");
    let z = zer as f32 / len as f32;
    println!("zer {zer} / {len} => {z}");

    (
        Decimal::try_from(p)
            .unwrap()
            .round_dp_with_strategy(6, rust_decimal::RoundingStrategy::MidpointAwayFromZero),
        Decimal::try_from(n)
            .unwrap()
            .round_dp_with_strategy(6, rust_decimal::RoundingStrategy::MidpointAwayFromZero),
        Decimal::try_from(z)
            .unwrap()
            .round_dp_with_strategy(6, rust_decimal::RoundingStrategy::MidpointAwayFromZero),
    )
}

#[rstest]
#[case(vec![-4, 3, -9, 0, 4, 1], (dec!(0.500000), dec!(0.333333), dec!(0.166667)))]
fn test_get_ratios(#[case] input: Vec<i32>, #[case] output: (Decimal, Decimal, Decimal)) {
    assert_eq!(get_ratios(&input), output);
}

fn count_pnz(arr: &[i32]) -> (u8, u8, u8) {
    arr.iter().fold((0, 0, 0), inc_count_pnz)
}

#[rstest]
#[case(vec![-4, 3, -9, 0, 4, 1], (3, 2, 1))]
fn test_count_pnz(#[case] input: Vec<i32>, #[case] output: (u8, u8, u8)) {
    assert_eq!(count_pnz(&input), output);
}

fn inc_count_pnz(prev: (u8, u8, u8), val: &i32) -> (u8, u8, u8) {
    match val.cmp(&0) {
        Ordering::Greater => (prev.0 + 1, prev.1, prev.2),
        Ordering::Less => (prev.0, prev.1 + 1, prev.2),
        Ordering::Equal => (prev.0, prev.1, prev.2 + 1),
    }
}

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
