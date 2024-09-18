// use easy::a281::word_capitalize_test;

// use easy::a236::boy_or_girl_test;

// use easy::a617::elephant_move;

// use easy::a148::insomnia;

mod easy;
fn main() {

    // let mut k_b = String::with_capacity(10);
    // let mut l_b = String::with_capacity(10);
    // let mut m_b = String::with_capacity(10);
    // let mut n_b = String::with_capacity(10);
    // let mut d_b = String::with_capacity(10);

    // std::io::stdin().read_line(&mut k_b).unwrap();
    // std::io::stdin().read_line(&mut l_b).unwrap();
    // std::io::stdin().read_line(&mut m_b).unwrap();
    // std::io::stdin().read_line(&mut n_b).unwrap();
    // std::io::stdin().read_line(&mut d_b).unwrap();


    // let k = k_b.trim().parse::<u8>().unwrap();
    // let l = l_b.trim().parse::<u8>().unwrap();
    // let m = m_b.trim().parse::<u8>().unwrap();
    // let n = n_b.trim().parse::<u8>().unwrap();
    // let d = d_b.trim().parse::<u64>().unwrap();



    let k =insomnia(2,3,4,5,24);
    println!("{}",k);
}

pub fn insomnia(k: u8, l: u8, m: u8, n: u8, d: u64) -> u64{


    if k==1 || l==1 || m ==1 || n ==1 {
        return d;
    }

    // subtract number of this multiples
    let kl = lcm(k , l);
    let km = lcm(k,m);
    let kn = lcm(k , n);

    let lm = lcm(l , m);
    let ln = lcm(l , n);

    let mn = lcm(m , n);

    let from_k = d / k as u64;
    let from_l = d / l as u64;
    let from_m = d / m as u64;
    let from_n = d / n as u64;

    println!("{} {} {} {}", from_k, from_l , from_m , from_n);

    let from_values = from_k+ from_l + from_m + from_n;

    let mut repeat = d / kl;



    repeat += d / km;
    repeat += d / kn ;
    repeat += d / lm ;
    repeat += d / ln ;
    repeat += d / mn ;
    println!("rp {}", repeat);

    return  from_values - repeat;
}


fn gcd(a: u8, b: u8)-> u64{
    if b!=0 {
        return  gcd(b, a%b);
    }
    else {
        return a as u64;
    }
}

fn lcm(a:u8, b:u8)-> u64 {
   (a * b) as u64 / gcd(a, b) 
}

