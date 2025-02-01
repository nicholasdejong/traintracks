use solver::{solve, Board, Rail::*};

fn main() {
    const X: usize = 7;
    const Y: usize = 9;
    let mut grid = [[None; X]; Y];

    // add the starting rails
    grid[0][4] = Some(NS);
    grid[0][6] = Some(SE);
    grid[2][6] = Some(NS);
    grid[6][2] = Some(NS);
    grid[8][2] = Some(WE);

    let col_hints = [2, 5, 5, 2, 5, 5, 7];
    let row_hints = [2, 2, 3, 7, 3, 3, 3, 3, 5];

    let board: Board<X, Y> = Board::new(grid, col_hints, row_hints);

    println!("Rail Count: {}", board.rail_count());

    if let Some(solution) = solve(board) {
        println!("{solution}");
    }
}
