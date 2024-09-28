
// a61
fn ultra_fast_math(s1: &str, s2: &str) -> String {
    let mut a: Vec<char> = Vec::with_capacity(s1.len());
    for (f1, f2) in s1.chars().zip(s2.chars()) {
        if f1 != f2 {
            a.push('1');
        } else {
            a.push('0');
        }
    }
    String::from_iter(a)
}

pub fn ultra_fast_math_test() {
   let mut s1 = String::with_capacity(30);
   std::io::stdin().read_line(&mut s1).unwrap();
   let mut s2 = String::with_capacity(30);
   std::io::stdin().read_line(&mut s2).unwrap();
   println!("{}", ultra_fast_math(s1.trim(), s2.trim()));
}
