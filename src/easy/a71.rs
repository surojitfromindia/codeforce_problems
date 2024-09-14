use std::io::stdin;


pub fn way_too_long(){
    let mut buffer = String::with_capacity(100);
    stdin().read_line(&mut buffer).unwrap();
    
    let word_count: u8 = buffer.trim().parse::<u8>().unwrap();
    let mut words : Vec<String> = Vec::with_capacity(word_count.into());
    buffer.clear();

    for _ in 0..word_count {
        stdin().read_line(&mut buffer).unwrap();
        let bf = buffer.trim();
        // direct calculate and storeif
        let len = bf.len();
        if len > 10 {
            words.push(shrink(bf, len));
        }
        else{
            words.push(bf.to_string());
        }
        // clear the buffer
        buffer.clear()
    }
    for c in words {
        print!("\n{}", c);
    }



}

#[inline]
fn shrink(lg: &str, str_len: usize)-> String{
    let f = lg.chars().nth(0).unwrap();
    let l = lg.chars().last().unwrap();
    return format!("{}{}{}",f, str_len-2, l);
}