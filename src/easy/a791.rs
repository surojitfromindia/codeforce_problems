fn big_bro(lima: u8, bob: u8) -> u64 {
    let mut year = 0;

    let mut lima_age: u64 = lima as u64;
    let mut bob_age: u64 = bob as u64;

    while lima_age <= bob_age {
        lima_age = lima_age * 3;
        bob_age = bob_age * 2;
        year = year + 1;
    }

    year
}

pub fn big_bro_test() {
    let mut buffer = String::with_capacity(10);
    std::io::stdin().read_line(&mut buffer).unwrap();

    let v: Vec<&str> = buffer.trim().split(" ").collect();
    let lima = v[0].parse::<u8>().unwrap();
    let bob = v[1].parse::<u8>().unwrap();
    println!("{}", big_bro(lima, bob))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn word_capitalize_test_1() {
        assert_eq!(big_bro(4, 7), 2);
        assert_eq!(big_bro(1, 10), 5);
    }
    // M.3^n >= P.2^n
    // 3/2^n >= P/N
}
