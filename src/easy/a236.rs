static ASCII_A: u8 = b'A';
static ASCII_a: u8 = b'a';

#[inline]
fn boy_or_girl(inp: String) -> bool {
    // we have 26 empty position of alphabet.
    let mut alphabet_buck: [u8; 26] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    // now covert the char to
    for c in inp.chars() {
        // we have captial letter
        let mut c_ascii = c as u8;
        if c_ascii < ASCII_a {
            // normalize A -> to 0, B to 1 etc
            c_ascii = c_ascii - ASCII_A;
        } else {
            // normalize a to 0, b to 1, c to 2 etc.
            // thus A or a update same index in the slice
            c_ascii = c_ascii - ASCII_a;
        }
        alphabet_buck[c_ascii as usize] = 1;
    }
    let odd_count = alphabet_buck.into_iter().filter(|x| *x == 1).count();
    odd_count % 2 ==0 // a girl
}

pub fn boy_or_girl_test() {
    let mut buffer = String::with_capacity(10);
    std::io::stdin().read_line(&mut buffer).unwrap();
    let is_girl = boy_or_girl(buffer.trim().to_string());

    if !is_girl {
        println!("IGNORE HIM!");
    } else {
        println!("CHAT WITH HER!")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn word_capitalize_test_1() {
        assert_eq!(boy_or_girl("wjmzbmr".to_string()), true);
        assert_eq!(boy_or_girl("xiaodao".to_string()), false);
    }
}
