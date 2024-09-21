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


pub fn magnets_test(){

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
