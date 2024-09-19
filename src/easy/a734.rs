

fn anton_and_danik(game: &str)-> &str{
    let mut anton_score : u32 = 0;
    let mut danik_score : u32 = 0;
    for c in game.chars() {
        if c == 'A' {
            anton_score+=1;
        }
        else{
            danik_score+=1;
        }
    }

    if anton_score > danik_score {
        "Anton"
    }
    else if danik_score > anton_score {
        "Danik"
    }
    else{
        "Friendship"
    }
}


fn anton_and_danik_test(){

    let mut c_str = String::with_capacity(10);
    std::io::stdin().read_line(&mut c_str).unwrap();

    let n = c_str.trim().parse::<usize>().unwrap();
    let mut game_str =   String::with_capacity(n);
    std::io::stdin().read_line(&mut game_str).unwrap();

    let l = anton_and_danik(&game_str.trim());

    println!("{}", l);
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn wrong_subtract_test() {
        assert_eq!(anton_and_danik("ADAAAA"), "Anton");
        assert_eq!(anton_and_danik("DDDAADA"),"Danik");
        assert_eq!(anton_and_danik("DADADA"),"Friendship");
    }
}
