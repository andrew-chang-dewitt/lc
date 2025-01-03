use rstest::rstest;

/*
 * Complete the 'get_ways' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts following parameters:
 *  1. INTEGER n
 *  2. LONG_INTEGER_ARRAY c
 */

#[rstest]
#[case((3, vec![8,3,1,2]), 3)]
#[case((4, vec![1,2,3]), 4)]
#[case((10, vec![2,5,3,6]), 5)]
#[case((166, vec![5,37,8,39,33,17,22,32,13,7,10,35,40,2,43,49,46,19,41,1,12,11,28]), 96_190_959)]
fn test_get_ways(#[case] input: (usize, Vec<usize>), #[case] expected: usize) {
    let (n, c) = input;
    assert_eq!(get_ways(n, &c), expected);
}

fn get_ways(n: usize, c: &[usize]) -> usize {
    let mut ways = vec![0; n + 1];
    ways[0] = 1;

    for coin in c {
        for j in *coin..(n + 1) {
            ways[j] = ways[j] + ways[j - coin];
        }
    }

    ways[n]
}

/*
 * # Proof
 *
 * This works because of the following:
 *
 * ## Optimal subproblems:
 *
 * Given an amount, n, and coin denominations, c,
 *   where n > 0
 *     and every c_i in c > 0
 *   get_ways returns the number of ways any combination
 *   of k_i c_i in c can be summed to equal n
 *
 * A solution, s, for some n and c looks like:
 *   s = { w_1, w_2, ..., w_s }, where
 *   w_i = (c_11, c_12, ..., c_1k_1, c_21, c_22, ... c_2k_2, ..., c_n1, c_n2, c_nk_n), where
 *   k_i is the number of occurences of c_i in the way, w_i
 *
 * Assume:
 *   for n & c, there exists a solution, s, containing all ways, w_i, that
 *   the values in c can be summed to equal n
 *
 * Hypothesis:
 *   there exists some value, n_o, where 1 < n_o <= ( n - some c_i ), whose solution contains a
 *   way, w_o, that is not included in any way, w_i, in s for n
 *
 * Then:
 *   TODO: somehow show that w_o must be in some w_i in s for n
 *
 */
