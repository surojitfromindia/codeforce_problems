 fn theater_squre(m:usize, n:usize, a: usize) -> usize {
   
    let m_wise_rem = m % a;
    let m_wise_req = (m / a) + if m_wise_rem > 0 { 1 } else { 0 };

    let n_wise_rem = n % a;
    let n_wise_req = (n / a) + if n_wise_rem > 0 { 1 } else { 0 };

    n_wise_req * m_wise_req
}

pub fn theater_squre_test(){
    let mut buffer = String::with_capacity(30);
    std::io::stdin().read_line(&mut buffer).unwrap();
    let v: Vec<usize> = buffer
        .trim()
        .split(" ")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let m = v[0];
    let n = v[1];
    let a = v[2];
    println!("{}", theater_squre(m, n, a));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn theater_squre_test() {
        
        assert_eq!(theater_squre(6,6,4), 4);
    }
}
