fn peyta_string(s1: &str, s2: &str) -> i8 {
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        match c1.to_ascii_lowercase().cmp(&c2.to_ascii_lowercase()) {
            std::cmp::Ordering::Less => return -1,
            std::cmp::Ordering::Greater => return 1,
            std::cmp::Ordering::Equal => continue,
        }
    }
    // If all characters are the same, return 0
    0
}

pub fn peyta_string_test(){
    let mut s1 = String::with_capacity(30);
    let mut s2 = String::with_capacity(30);
    std::io::stdin().read_line(&mut s1).unwrap();
    std::io::stdin().read_line(&mut s2).unwrap();
    println!("{}", peyta_string(s1.as_str(),s2.as_str()));
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn peyta_string_test() {
        assert_eq!(peyta_string("aaaa", "aaaa"), 0);
       
    }
}
