// a41
fn translation<'a>(s: &'a str, t: &'a str) -> &'a str {
    if s.len() != t.len() {
        "NO"
    } else {
        for (cs, ct) in s.chars().zip(t.chars().rev()) {
            if cs != ct {
                return "NO";
            }
        }
        "YES"
    }
}

pub fn translation_test() {
    let mut s: String = String::with_capacity(10);
    std::io::stdin().read_line(&mut s).unwrap();

    let mut t: String = String::with_capacity(10);
    std::io::stdin().read_line(&mut t).unwrap();
    println!("{}", translation(s.trim(), t.trim()));
}
