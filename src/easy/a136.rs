// a136
fn present(senders: Vec<u8>) -> String {
    let mut rec = vec![0; senders.len()];
    for (inx, s) in senders.into_iter().enumerate() {
        rec[(s - 1) as usize] = (inx + 1) as u8
    }
    rec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}
pub fn present_test() {
    let mut s1 = String::with_capacity(3);
    std::io::stdin().read_line(&mut s1).unwrap();
    let n = s1.trim().parse::<usize>().unwrap();
    let mut s2 = String::with_capacity(n * 2);
    std::io::stdin().read_line(&mut s2).unwrap();

    let c: Vec<u8> = s2
        .trim()
        .split(" ")
        .map(|z| z.parse::<u8>().unwrap())
        .collect();

    println!("{}", present(c))
}
