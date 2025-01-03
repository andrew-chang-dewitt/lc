use std::convert::TryFrom;

/*
 * Complete the 'highestValuePalindrome' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. STRING s
 *  2. INTEGER n
 *  3. INTEGER k
 */

#[test]
fn test_highest_value_palindrome() {
    let cases = [("1231", 3, "9339"), ("1231", 3, "9339")];

    for (s, k, expected) in cases {
        assert_eq!(
            highest_value_palindrome(s, s.len().try_into().unwrap(), k),
            expected
        );
    }
}

fn highest_value_palindrome(s: &str, _n: i32, _k: i32) -> String {
    // first check if palindrome, if so get it's value
    // set greatest so far as 0 (or value of s if it's a palindrome)
    let highest: (&str, usize) = (s, palindrome_value(s));

    return match k {
        0 => {
            println!("highest is {highest:#?}");
            format!("{}", highest.0).to_string()
        }
        1 => {
            let max = get_one_permutations(s)
                .map(|p| (p, palindrome_value(p)))
                .max();
        }
        _ => {}
    };
    // then, decrement k and make list of all palindromes 1
    // change away from s and their values
    // compare those values to greatest so far, updating if greater
}

fn get_one_permutations(s: &str) -> Vec<String> {
    let l = s.len() - 1;
    let mut i: usize = 0;
    let mut j: usize = l;
    let v: Vec<char> = s.chars().collect();
    let mut r: Vec<String> = Vec::new();

    while i < j {
        if v[i] != v[j] {
            let h = if v[i] > v[j] { v[i] } else { v[j] };
            let mut n: Vec<char> = Vec::new();
            for k in 0..l {
                if k == i || k == j {
                    n.push(h);
                } else {
                    n.push(v[k]);
                }
            }

            r.push(n.iter().collect());
        }

        i += 1;
        j -= 1;
    }

    r
}

fn palindrome_value(s: &str) -> usize {
    if is_palindrome(s) {
        return get_value(s);
    }

    return 0;
}

fn get_value(s: &str) -> usize {
    s.chars()
        .map(|c| usize::try_from(c.to_digit(10).unwrap()).unwrap())
        .sum()
}

fn is_palindrome(s: &str) -> bool {
    let mut i: usize = 0;
    let mut j: usize = s.len() - 1;
    let v: Vec<char> = s.chars().collect();

    while i < j {
        if v[i] != v[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }

    return true;
}
