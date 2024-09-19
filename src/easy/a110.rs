pub fn near_lucky_number() {
    let mut n = String::with_capacity(10);
    std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u64>().unwrap();
    let mut count = 0;
    for i in n.to_string().chars() {
        if i == '4' || i == '7' {
            count += 1;
        }
    }
    if count == 4 || count == 7 {
        println!("YES");
    } else {
        println!("NO");
    }
}
