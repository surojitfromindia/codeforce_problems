// a271
fn is_year_num_distinct(year: u16) -> bool {
    let mut y = year;
    // first
    let first_dig = y / 1000;
    let sec_dig = (y / 100) % 10;
    let third_dig = (y / 10) % 10;
    let for_dig = y % 10;

    if first_dig == sec_dig
        || first_dig == third_dig
        || first_dig == for_dig
        || sec_dig == third_dig
        || sec_dig == for_dig
        || third_dig == for_dig
    {
       return false
    }
    true
}

fn beautiful_year_test(){
    let mut b1 = String::with_capacity(10);
    std::io::stdin().read_line(&mut b1).unwrap();
    let mut year = b1.trim().parse::<u16>().unwrap();
    
    loop {
        year+=1;
        if is_year_num_distinct(year){
            break;
        }
    }
    println!("{}", year)
}
