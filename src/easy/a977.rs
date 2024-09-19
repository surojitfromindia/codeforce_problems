fn wrong_subtract(number: u64, repeat: u8) -> u64 {
    let mut n = number;

    for _ in 0..repeat {
        if n % 10 == 0 {
            n /= 10;
        } else {
            n -= 1
        }
    }
    n
}

pub fn wrong_subtract_test() {
    let mut buffer = String::with_capacity(10);
    std::io::stdin().read_line(&mut buffer).unwrap();

    let v: Vec<u64> = buffer
        .trim()
        .split(" ")
        .map(|z| z.parse::<u64>().unwrap())
        .collect();
    let n = v[0];
    let k = v[1] as u8; // could overflow but ok as test only give k from 0 to 50.

    println!("{}", wrong_subtract(n, k));
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn wrong_subtract_test() {
        assert_eq!(wrong_subtract(512, 4), 50);
        assert_eq!(wrong_subtract(1000000000, 9), 1);
    }
}
