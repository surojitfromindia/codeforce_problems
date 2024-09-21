//a467
pub fn george_and_accommodation() {
    let mut s1 = String::with_capacity(5);
    std::io::stdin().read_line(&mut s1).unwrap();
    let s1_num = s1.trim().parse::<u8>().unwrap();

    let mut count = 0;
    for _ in 0..s1_num {
        let mut s2 = String::with_capacity(5);
        std::io::stdin().read_line(&mut s2).unwrap();
        let r : Vec<u8>= s2.trim().split(" ").map(|z| z.parse::<u8>().unwrap()).collect();
        let empty = r[1]- r[0];
        if empty >=2 {
            count+=1;
        }

    }

    println!("{}",count);

}
