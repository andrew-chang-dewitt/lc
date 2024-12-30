use rstest::rstest;

/*
 * Complete the 'getWays' function below.
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
    assert_eq!(getWays(n, &c), expected);
}

fn getWays(n: usize, c: &[usize]) -> usize {
    let mut ways = vec![0; n + 1];
    ways[0] = 1;

    for coin in c {
        for j in *coin..(n + 1) {
            ways[j] = ways[j] + ways[j - coin];
        }
    }

    ways[n]
}
