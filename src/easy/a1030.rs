// a1030
fn easy_problem<'a>(votes: Vec<u8>) -> &'a str {
    let sum: u8 = votes.iter().sum();
    if sum == 0 {
        return "EASY";
    }
    "HARD"
}
pub fn easy_problem_test() {
    let mut s1 = String::with_capacity(5);
    std::io::stdin().read_line(&mut s1).unwrap();
    let s1_num = s1.trim().parse::<u8>().unwrap();
    let mut s2 = String::with_capacity((s1_num*2) as usize);
    std::io::stdin().read_line(&mut s2).unwrap();
    let votes : Vec<u8>= s2.trim().split(" ").map(|z| z.parse::<u8>().unwrap()).collect();
    println!("{}", easy_problem(votes));

}
