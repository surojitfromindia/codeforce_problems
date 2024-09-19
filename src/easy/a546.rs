fn word(inp: String) -> String {
    // if smaller count is greater then lower count. convert to small

    let mut small_count: u8 = 0;
    let mut upper_count: u8 = 0;

    for c in inp.chars() {
        // small case start from 'a'.
        let ascii_c = c as u8;
        if ascii_c < b'a' {
            upper_count += 1;
        } else {
            small_count += 1
        }
    }

    //   println!("s {} u {} ", small_count, upper_count);

    if small_count >= upper_count {
        inp.to_ascii_lowercase()
    } else {
        inp.to_ascii_uppercase()
    }
}

pub fn word_test() {
    let mut inp = String::with_capacity(10);
    std::io::stdin().read_line(&mut inp).unwrap();
    println!("{}", word(inp.trim().to_string()));
}
