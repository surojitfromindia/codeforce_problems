

fn word_capitalize(mut inp :  String)->String{
    unsafe {
        let n = inp.as_bytes_mut();
        n[0] = n[0].to_ascii_uppercase();
        inp
    }
}

pub fn word_capitalize_test(){
    let mut buffer = String::with_capacity(10);
    std::io::stdin().read_line(&mut buffer).unwrap();
    let res = word_capitalize(buffer.trim().to_string());
    println!("{}",res);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn word_capitalize_test_1() {
        assert_eq!(word_capitalize("hi".to_string()),"Hi".to_string());
    }
}