
// 282A
pub fn bit(){
    let mut n_buff = String::with_capacity(5);
    std::io::stdin().read_line(&mut n_buff).unwrap();
    let n = n_buff.trim().parse::<u16>().unwrap();

    let mut x = 0;

    let mut st_buff = String::with_capacity(5);

    for _ in 0..n {
        std::io::stdin().read_line(&mut st_buff).unwrap();
        let s_trm = st_buff.trim();
        // if last and first char both are not zero then we 
        // have a true
        let f = s_trm.chars().nth(0).unwrap();
        let l = s_trm.chars().last().unwrap();

        // println!("{}, {}", f, l);
        let sum = (f as u8) + (l as u8);
        // if sum of (X - 88, - -> 45, + -> 43)

        if sum == 131 {
            x+=1;
        }
        else{
            x-=1;
        }

        st_buff.clear();
    }

    println!("{}",x)
}