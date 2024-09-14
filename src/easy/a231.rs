
pub fn team(){
    let mut n_buff = String::with_capacity(5);
    std::io::stdin().read_line(&mut n_buff).unwrap();

    let n = n_buff.trim().parse::<u16>().unwrap();

    let mut st_buff = String::with_capacity(8);
    // we have n number of inputs
    let mut counter = 0;
    for _ in 0..n {
        std::io::stdin().read_line(&mut st_buff).unwrap();
        let s_trm = st_buff.trim();
        // if last and first char both are not zero then we 
        // have a true
        let f = s_trm.chars().nth(0).unwrap().to_digit(10).unwrap();
        let s = s_trm.chars().nth(2).unwrap().to_digit(10).unwrap();
        let l = s_trm.chars().last().unwrap().to_digit(10).unwrap();
        let sum = f + s + l;

        if sum > 1 {
            counter+=1;
        }

        st_buff.clear();
    }

    println!("{}",counter);
}
