// domino pilling.
/* You are given a rectangular board of M × N squares. Also you are given an unlimited number of standard domino pieces of 2 × 1 squares. You are allowed to rotate the pieces. You are asked to place as many dominoes as possible on the board so as to meet the following conditions:

1. Each domino completely covers two squares.

2. No two dominoes overlap.

3. Each domino lies entirely inside the board. It is allowed to touch the edges of the board.

Find the maximum number of dominoes, which can be placed under these restrictions.

Input
In a single line you are given two integers M and N — board sizes in squares (1 ≤ M ≤ N ≤ 16).

Output
Output one number — the maximal number of dominoes, which can be placed. */


fn domino_pilling(m: u8, n: u8) -> u8 {
    // another, 6, 5,   divide small side 5 / 2 = 2.
    // -> 6*3 = 18
    // remainder is 1, so we can have lay out even 1
    // 6/2 = 3-> 3* 1 = 3

    let max_side = std::cmp::max(m, n);
    let max_side_repeat = max_side / 2;

    let short_side = std::cmp::min(m, n);
    let short_side_repeat = short_side / 2;
    let short_side_remain = short_side % 2;

    let t = (max_side * short_side_repeat) + (short_side_remain * max_side_repeat);
    return t;
}

pub fn run_domino_pilling() {
    let mut buffer = String::with_capacity(30);
    std::io::stdin().read_line(&mut buffer).unwrap();
    let v: Vec<u8> = buffer
        .trim()
        .split(" ")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    let m = v[0];
    let n = v[1];
    println!("{}",domino_pilling(m, n));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn domino_pilling_test() {
        assert_eq!(domino_pilling(2, 4), 4);
        assert_eq!(domino_pilling(3, 3), 4);
    }
}
