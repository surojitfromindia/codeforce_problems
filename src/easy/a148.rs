fn insomnia(k: u64, l: u64, m: u64, n: u64, d: u64) -> u64 {
    if k == 1 || l == 1 || m == 1 || n == 1 {
        return d;
    }

    // todo: another solution using inclusion-exclusion principle
    // https://codeforces.com/contest/148/submission/11987935
    let mut count = 0;
    for i in 1..=d {
        if i % k == 0 || i % l == 0 || i % m == 0 || i % n == 0 {
            count += 1;
        }
    }
    count
}

pub fn insomnia_test() {
    let mut k_b = String::with_capacity(10);
    let mut l_b = String::with_capacity(10);
    let mut m_b = String::with_capacity(10);
    let mut n_b = String::with_capacity(10);
    let mut d_b = String::with_capacity(10);

    std::io::stdin().read_line(&mut k_b).unwrap();
    std::io::stdin().read_line(&mut l_b).unwrap();
    std::io::stdin().read_line(&mut m_b).unwrap();
    std::io::stdin().read_line(&mut n_b).unwrap();
    std::io::stdin().read_line(&mut d_b).unwrap();

    let k = k_b.trim().parse::<u8>().unwrap();
    let l = l_b.trim().parse::<u8>().unwrap();
    let m = m_b.trim().parse::<u8>().unwrap();
    let n = n_b.trim().parse::<u8>().unwrap();
    let d = d_b.trim().parse::<u64>().unwrap();

    let k = insomnia(k as u64, l as u64, m as u64, n as u64, d);
    println!("{}", k);
}
