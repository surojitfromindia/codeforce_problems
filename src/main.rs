mod easy;
fn main() {
    magnets_test();
}





// a344
fn magnets(inp: Vec<String>)-> u64{
    // it will create a group if the next read value is not same as last read value
    let mut break_count = 0; // will have at least single group.
    for i in 0..inp.len()-1{
        if inp[i] != inp[i+1]{
            break_count+=1;
        }
    }
    break_count+1
}


fn magnets_test(){

    let mut s1 = String::with_capacity(5);
    std::io::stdin().read_line(&mut s1).unwrap();
    let s1_num = s1.trim().parse::<u32>().unwrap();

    let mut inp: Vec<String> = Vec::with_capacity(s1_num as usize) ;
    
    for _ in 0..s1_num {
        let mut s2 = String::with_capacity(5);
        std::io::stdin().read_line(&mut s2).unwrap();
        inp.push(s2.trim().to_string());
    }
    println!("{}", magnets(inp));
}


#[cfg(test)]
mod test{

    use super::*;
    #[test]
    fn magnets_test() {
        let an = vec!["10","10","10","01","10","10"].into_iter().map(|z| z.to_owned()).collect();
        assert_eq!(magnets(an),3);
        let an2 = vec!["01","01","10","10"].into_iter().map(|z|  z.to_owned()).collect();
        assert_eq!(magnets(an2),2);

    }
}








//a467
fn george_and_accommodation() {
    let mut s1 = String::with_capacity(5);
    std::io::stdin().read_line(&mut s1).unwrap();
    let s1_num = s1.trim().parse::<u8>().unwrap();

    let mut count = 0;
    for _ in 0..s1_num {
        let mut s2 = String::with_capacity(5);
        std::io::stdin().read_line(&mut s2).unwrap();
        let r : Vec<u8>= s2.trim().split(" ").map(|z| z.parse::<u8>().unwrap()).collect();
        let empty = r[1]- r[0];
        if empty >=2 {
            count+=1;
        }

    }

    println!("{}",count);

}

// a1030
fn easy_problem<'a>(votes: Vec<u8>) -> &'a str {
    let sum: u8 = votes.iter().sum();
    if sum == 0 {
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



