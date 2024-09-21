use std::collections::HashSet;

mod easy;
mod easy_b;
fn main() {
    hulk_test();
}



fn hulk_test(){
    let mut s1 = String::with_capacity(30);
    std::io::stdin().read_line(&mut s1).unwrap();
    let n = s1.trim().parse::<usize>().unwrap();

    let mut res = String::with_capacity(10);

    // start case from > 1
    for i in 1..n {
        if i  % 2 != 0 {
           res.push_str("I hate that ");
        }
        else{
           res.push_str("I love that ");

        }
    }
    // if n is odd
    if n & 1 ==1 {
        res.push_str("I hate it ");
    }
    else{
        res.push_str("I love it ");
    }
    println!("{}", res.trim());
}



// a705
fn horseshoe(s1: u64, s2: u64, s3: u64, s4: u64)-> usize{
    let mut hs = HashSet::new();
    hs.insert(s1);
    hs.insert(s2);
    hs.insert(s3);
    hs.insert(s4);
    4 - hs.len()
}
fn horseshoe_test(){
    let mut s2 = String::with_capacity(10);
    std::io::stdin().read_line(&mut s2).unwrap();

    let c  = s2
    .trim()
    .split(" ")
    .map(|z| z.parse::<u64>().unwrap()).collect::<Vec<_>>();

    println!("{}", horseshoe(c[0], c[1], c[2], c[3]));
}



// a61
fn ultra_fast_math(s1 : &str, s2: &str)->String{
    let mut a : Vec<char> = Vec::with_capacity(s1.len());
    for (f1,f2) in s1.chars().zip(s2.chars()) {
        if f1!=f2 {
            a.push('1');
        }
        else{
            a. push('0');
        }
    }
    String::from_iter(a)
}

fn ultra_fast_math_test(){
    let mut s1 = String::with_capacity(30);
    std::io::stdin().read_line(&mut s1).unwrap();
    let mut s2 = String::with_capacity(30);
    std::io::stdin().read_line(&mut s2).unwrap();
    println!("{}", ultra_fast_math(s1.trim(), s2.trim()));
}


// b200
fn drinks_test(){
    let mut s1 = String::with_capacity(3);
    std::io::stdin().read_line(&mut s1).unwrap();
    let n = s1.trim().parse::<usize>().unwrap();
    let mut s2 = String::with_capacity(n * 2);
    std::io::stdin().read_line(&mut s2).unwrap();

    let c : u64 = s2
    .trim()
    .split(" ")
    .map(|z| z.parse::<u64>().unwrap())
    .sum();

    println!("{}", c as f32 / n as f32);
}
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
fn present_test() {
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

