#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Function to calculate the minimum number of moves for the king
fn min_king_moves(start: Point, target: Point) -> i32 {
    let dx = (target.x - start.x).abs();
    let dy = (target.y - start.y).abs();

    // Minimum of diagonal moves plus any extra straight moves
    let diagonal_moves = dx.min(dy);
    let remaining_straight_moves = (dx - diagonal_moves) + (dy - diagonal_moves);

    diagonal_moves + remaining_straight_moves
}

fn main() {
    let start = Point { x: 1, y: 8 };
    let target = Point { x:8, y: 1 };

    let moves = min_king_moves(start, target);
    println!("Minimum number of moves: {}", moves);
}
