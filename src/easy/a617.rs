
pub fn elephant_move(to_pos: u64)-> u64{
    let mut moves = 0;
    let mut e_pos : u64 = 0;      
      let diff =   to_pos - e_pos;

    while to_pos > e_pos {
        // remaining steps are less than 4
        if to_pos - e_pos <= 4 {
            moves+=1;
            e_pos+=diff;
            // println!("less {} e_pos {}", diff, e_pos);

        }
        else{
            // we need to take all 5 steps
            let step_5_moves = (to_pos - e_pos) / 5;
            // println!("s {} {} {}", step_5_moves, to_pos, e_pos);
            moves+=step_5_moves;
            // now subtract the distance
            e_pos+= 5 * step_5_moves;
        }
    }
    moves
}
fn elephant_move_test(){
    let mut n_buff = String::with_capacity(5);
    std::io::stdin().read_line(&mut n_buff).unwrap();
    let n = n_buff.trim().parse::<u64>().unwrap();

    println!("{}", elephant_move(n));
}