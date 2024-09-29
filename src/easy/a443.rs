use std::collections::HashSet;


// a443
fn anton_and_letters(f_set: &str) -> usize {
    let mut hash_set: HashSet<u8> = HashSet::new();

    for c in f_set.chars() {
        let c_n = c as u8;
        if c_n >= b'A' && c_n <= b'z' {
            hash_set.insert(c_n);
        }
    }
    return hash_set.len();
}
pub fn anton_and_letters_test() {
    let mut inp = String::with_capacity(10);
    std::io::stdin().read_line(&mut inp).unwrap();
    println!("{}", anton_and_letters(inp.trim()));
}
