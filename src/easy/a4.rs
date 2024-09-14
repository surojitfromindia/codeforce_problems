use std::io::stdin;

pub fn watermelon() {
    // if the inp is even then print yes
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let inp = buffer.trim().parse::<u8>().unwrap();
    // handle the case for 2.
    if inp == 2 {
        print!("NO");
    } else if inp % 2 == 0 {
        print!("YES")
    } else {
        print!("NO")
    }
}

