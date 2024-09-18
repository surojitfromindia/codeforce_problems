
fn stones_on_table(inp: String) -> usize {
    let b: Vec<char> = inp.trim().chars().collect();
    let mut old_point: char = b[0];
    // now iterate
    let mut rm = 0;
    for i in 1..b.len() {
        if b[i] == old_point {
            rm += 1;
        } else {
            // move the old point to this.
            old_point = b[i];
        }
    }
    return rm;
}

fn stones_on_table_test (){
    // read the fake input.
    let mut buffer2 = String::with_capacity(10);
    std::io::stdin().read_line(&mut buffer2).unwrap();
    let mut buffer = String::with_capacity(10);
    std::io::stdin().read_line(&mut buffer).unwrap();
    println!("{}", stones_on_table(buffer));
}


