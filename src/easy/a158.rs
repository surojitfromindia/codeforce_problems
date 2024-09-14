use std::io::stdin;

pub fn next_round() {
    let mut buffer = String::with_capacity(3);
    stdin().read_line(&mut buffer).unwrap();

    // this is how we split . then map to u8
    let v: Vec<u8> = buffer
        .trim()
        .split(" ")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();
    let n = v[0];
    let k = v[1];

    // we will have n numbers separated by 1 space, the number will be upto 100 (3 char)
    // so we need max 1+3 to store a single number in string.
    let mut s_buffer = String::with_capacity((n * 4).into());
    stdin().read_line(&mut s_buffer).unwrap();

    // the string has been read
    let mut count = 0;
    let scores: Vec<u8> = s_buffer
        .trim()
        .split(" ")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    let compare_with = scores[(k - 1) as usize];

    // println!("{}", compare_with);

    for sc in scores {
        if sc > 0 && sc >= compare_with {
            count += 1;
        }
    }

    println!("{}", count);
}
