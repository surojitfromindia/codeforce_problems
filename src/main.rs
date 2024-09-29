mod easy;
mod easy_b;

fn main() {
    hq9_test();
}

// a133
fn hq9_test() {
    let mut inp = String::with_capacity(10);
    std::io::stdin().read_line(&mut inp).unwrap();

    let mut dx = false;
    for c in inp.trim().chars() {
        if c == 'H' || c == 'Q' || c == '9' {
            dx = true
        }
    }
    println!("{}", if dx { "YES" } else { "NO" });
}
