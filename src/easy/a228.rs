use std::collections::HashSet;

// a228
fn horseshoe(s1: u64, s2: u64, s3: u64, s4: u64) -> usize {
    let mut hs = HashSet::new();
    hs.insert(s1);
    hs.insert(s2);
    hs.insert(s3);
    hs.insert(s4);
    4 - hs.len()
}
pub fn horseshoe_test() {
    let mut s2 = String::with_capacity(10);
    std::io::stdin().read_line(&mut s2).unwrap();

    let c = s2
        .trim()
        .split(" ")
        .map(|z| z.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    println!("{}", horseshoe(c[0], c[1], c[2], c[3]));
}
