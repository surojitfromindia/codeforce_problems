pub fn insomnia(k: u8, l: u8, m: u8, n: u8, d: u64) -> u64{


    if k==1 || l==1 || m ==1 || n ==1 {
        return d;
    }

    // subtract number of this multiples
    let kl = k * l;
    let km = k * m;
    let kn = k * n;

    let lm = l * m;
    let ln = l * n;

    let mn = m * n;

    let from_k = d / k as u64;
    let from_l = d / l as u64;
    let from_m = d / m as u64;
    let from_n = d / n as u64;

    println!("{} {} {} {}", from_k, from_l , from_m , from_n);

    let from_values = from_k+ from_l + from_m + from_n;

    let mut repeat = d / kl as u64;
    repeat += d / km as u64;
    repeat += d / kn as u64;
    repeat += d / lm as u64;
    repeat += d / ln as u64;
    repeat += d / mn as u64;

    println!("d {} r {}", from_values, repeat);

    return  from_values - repeat;
}
