fn beautiful_matrix(row_pos: u8, col_pos: u8) -> u8 {
    // it is a 5  x 5 matrix.
    // the row should be 2 (0 based)
    let abs_diff_row = 2u8.abs_diff(row_pos);
    let abs_diff_col = 2u8.abs_diff(col_pos);

    abs_diff_col + abs_diff_row
}

pub fn run_beautiful_matrix() {
    let mut buffer = String::with_capacity(30);
    let mut rc = (0, 0);
    'row: for r in 0..5 {
        std::io::stdin().read_line(&mut buffer).unwrap();
        let row: Vec<u8> = buffer
            .trim()
            .split(" ")
            .map(|x| x.parse::<u8>().unwrap())
            .collect();
        // loop over row to find if the 1 is there
        for (c, col) in row.into_iter().enumerate() {
            if col == 1 {
                rc = (r, c);
                break 'row;
            }
        }
        buffer.clear();
    }
    println!("{}", beautiful_matrix(rc.0, rc.1 as u8));
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn beautiful_matrix_test() {
        assert_eq!(beautiful_matrix(1, 4), 3);
        assert_eq!(beautiful_matrix(2, 1), 1);
    }
}
