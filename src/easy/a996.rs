// a996
fn hit_the_lottery(money: u64)-> u64{
    let mut rm = money;
    let mut c = 0;

    while rm > 0 {
        if rm >=100 {
            rm-=100;
        }
        else if rm >=20 {
            rm-=20;
        }
        else if rm >=10 {
            rm-=10;
        }
        else if rm >=5 {
            rm-=5;
        }
        else{
            rm-=1;
        }
        c+=1;
    }
    c
}

pub fn hit_the_lottery_test(){
    let mut s1 = String::with_capacity(10);
    std::io::stdin().read_line(&mut s1).unwrap();
    let n = s1.trim().parse::<u64>().unwrap();
    println!("{}", hit_the_lottery(n));
}

