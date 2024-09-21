// a677
fn vanya_and_fence(h: u16, heights: Vec<u16>) -> usize {
    let hv_to_bend = heights.iter().filter(|&x| *x > h).count();
    let st = heights.len() - hv_to_bend;
    st + (hv_to_bend * 2)
}
fn vanya_and_fence_test() {
    let mut b1 = String::with_capacity(10);
    std::io::stdin().read_line(&mut b1).unwrap();
    let v1: Vec<u16> = b1
        .trim()
        .split(" ")
        .map(|x| x.parse::<u16>().unwrap())
        .collect();
    let n = v1[0];
    let h = v1[1];
    let mut b2 = String::with_capacity(n as usize * 2);
    std::io::stdin().read_line(&mut b2).unwrap();
    let heights: Vec<u16> = b2
        .trim()
        .split(" ")
        .map(|x| x.parse::<u16>().unwrap())
        .collect();
    println!("{}", vanya_and_fence(h, heights));
}
