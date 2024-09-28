// a705
pub fn hulk_test() {
    let mut s1 = String::with_capacity(30);
    std::io::stdin().read_line(&mut s1).unwrap();
    let n = s1.trim().parse::<usize>().unwrap();

    let mut res = String::with_capacity(10);

    // start case from > 1
    for i in 1..n {
        if i % 2 != 0 {
            res.push_str("I hate that ");
        } else {
            res.push_str("I love that ");
        }
    }
    // if n is odd
    if n & 1 == 1 {
        res.push_str("I hate it ");
    } else {
        res.push_str("I love it ");
    }
    println!("{}", res.trim());
}
