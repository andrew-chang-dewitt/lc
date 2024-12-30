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
fn test_get_ways(#[case] input: (isize, Vec<isize>), #[case] expected: isize) {
    let (n, c) = input;
    assert_eq!(getWays(n, &c), expected);
}

fn getWays(n: isize, c: &[isize]) -> isize {
    // sort currencies from smallest to largest
    let mut s = vec![0; c.len()];
    s.clone_from_slice(c);
    s.sort();
    // then recur over denoms to generate ways
    let ways = change(n.try_into().unwrap(), &s);
    println!("ways: {ways:#?}");
    // then count the ways & return
    ways.len().try_into().unwrap()
}

fn change(amount: isize, denoms: &[isize]) -> Vec<Vec<isize>> {
    if amount == 0 {
        println!("change({amount}, {denoms:?}) --> [[],]");
        return vec![vec![]];
    } else if amount < 0 || denoms.len() == 0 {
        println!("change({amount}, {denoms:?}) --> []");
        return vec![];
    } else {
        let mut r = vec![];

        for mut way in change(amount - denoms[0], denoms) {
            way.insert(0, denoms[0]);
            r.push(way);
        }

        r.append(&mut change(amount, &denoms[1..]));
        println!("change({amount}, {denoms:?}) --> {r:?}");

        r
    }
}
