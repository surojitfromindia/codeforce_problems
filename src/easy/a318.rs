
// a318
fn even_odds(n: u64, k: u64) -> u64 {
    let odd_numbers_count = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };
    // so, if the odd numbers comes first and the k is in the range of odd number count it is easy
    // {1,3,5,7,9,2,4,6,8}
    // k is 4, so the odd number is 4th which is 7 (2*n - 1)
    // if in range is inside even k = 6, then even number start from 6 -  odd_numbers_count which
    // is k_dash

    let b = if k <= odd_numbers_count {
        (2 * k) - 1
    } else {
        let k_dash = k - odd_numbers_count;
        2 * k_dash
    };
    b
}
fn even_odds_test() {
    let mut inp = String::with_capacity(10);
    std::io::stdin().read_line(&mut inp).unwrap();
    let v: Vec<u64> = inp
        .trim()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    println!("{}", even_odds(v[0], v[1]))
}

