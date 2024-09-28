

// b200
pub fn drinks_test() {
    let mut s1 = String::with_capacity(3);
    std::io::stdin().read_line(&mut s1).unwrap();
    let n = s1.trim().parse::<usize>().unwrap();
    let mut s2 = String::with_capacity(n * 2);
    std::io::stdin().read_line(&mut s2).unwrap();

    let c: u64 = s2
        .trim()
        .split(" ")
        .map(|z| z.parse::<u64>().unwrap())
        .sum();

    println!("{}", c as f32 / n as f32);
}