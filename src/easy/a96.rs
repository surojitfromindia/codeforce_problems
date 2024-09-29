
fn peyta_and_football(gp: &str)-> bool {
    let mut con = 1;
    let cs : Vec<char>= gp.chars().collect();
    for i in 0..gp.len() - 1 {
        if cs[i] == cs[i + 1] {
            con += 1;
        } else {
            con = 1;
        }
        if con == 7 {
            return true;
        }
    }
    return false
}

pub fn peyta_and_football_test(){
    let mut inp = String::with_capacity(10);
    std::io::stdin().read_line(&mut inp).unwrap();
   let n = peyta_and_football(inp.trim());

   println!("{}", if n {"YES"} else {"NO"});
}
