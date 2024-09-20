mod easy;
fn main() {
    easy_problem_test();
}

// a1030
fn easy_problem<'a>(votes: Vec<u8>) -> &'a str {
    let sum: u8 = votes.iter().sum();
    if (sum as usize) <= votes.len()/2 {
        return "EASY";
    }
    "HARD"
}
fn easy_problem_test() {
    let mut s1 = String::with_capacity(5);
    std::io::stdin().read_line(&mut s1).unwrap();
    let s1_num = s1.trim().parse::<u8>().unwrap();
    let mut s2 = String::with_capacity((s1_num*2) as usize);
    std::io::stdin().read_line(&mut s2).unwrap();
    let votes : Vec<u8>= s2.trim().split(" ").map(|z| z.parse::<u8>().unwrap()).collect();
    println!("{}", easy_problem(votes));

}


// b266
fn queue_at_school(inp: String, sec: u16) -> String {
    let mut que: Vec<char> = inp.chars().collect();
    'e_loop: for _ in 0..sec {
        let mut i = 0;
        // swap if left is a girl
        let mut any_swap_occurs = false;
        while i < que.len() - 1 {
            if que[i] == 'B' && que[i + 1] == 'G' {
                // swap
                que.swap(i + 1, i);
                any_swap_occurs = true;
                i += 2; // skip the next compare.
                continue;
            }
            i += 1;
        }
        // i can break the loop here, if  swap does not occurred in this loop
        // thus it mean an arrangement has already taken place.
        if !any_swap_occurs {
            break 'e_loop;
        }
    }
    String::from_iter(que)
}
fn queue_at_school_test() {
    let mut s1 = String::with_capacity(5);
    std::io::stdin().read_line(&mut s1).unwrap();
    let s1_v: Vec<u16> = s1
        .trim()
        .split(" ")
        .into_iter()
        .map(|z| z.parse::<u16>().unwrap())
        .collect();
    let str_len = s1_v[0];
    let sec = s1_v[1];

    let mut s2 = String::with_capacity((str_len + 2) as usize);
    std::io::stdin().read_line(&mut s2).unwrap();
    println!("{}", queue_at_school(s2.trim().to_string(), sec));
}

