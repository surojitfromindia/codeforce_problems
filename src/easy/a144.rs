//  arrival of the general
 fn arrival_of_gen(v: Vec<u8>) -> u32 {
    let ln = v.len();

    if ln == 1 || v.is_empty() {
        return 0;
    }

    let mut max = v[0];
    let mut max_pos = 0;

    let mut min = v[0];
    let mut min_pos = 0;

    for (inx, c) in v.into_iter().enumerate() {
        if c > max {
            max = c;
            max_pos = inx;
        } else if c <= min {
            min = c;
            min_pos = inx;
        }
    }
    // first shift biggest to the left. which increase any pos of other element by 1
    // so the smaller also shift to right by 1 (if min_pos < max_pos)

    let shift = if min_pos < max_pos { 1 } else { 0 };
    return (max_pos + (ln - 1 - (min_pos + shift))) as u32;
}

pub fn arrival_of_gen_test(){
    let mut s1 = String::with_capacity(3);
    std::io::stdin().read_line(&mut s1).unwrap();
    let n = s1.trim().parse::<usize>().unwrap();
    let mut s2 = String::with_capacity(n * 2);
    std::io::stdin().read_line(&mut s2).unwrap();

    let c: Vec<u8> = s2
        .trim()
        .split(" ")
        .map(|z| z.parse::<u8>().unwrap())
        .collect();
    println!("{}", arrival_of_gen(c));
}

