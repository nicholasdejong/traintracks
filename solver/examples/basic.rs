use solver::{solve, Board, Rail::*};

fn main() {
    const X: usize = 3;
    const Y: usize = 3;
    let mut grid = [[None; X]; Y];

    // add the starting rails
    grid[0][0] = Some(WE);
    grid[2][0] = Some(WE);

    let col_hints = [2, 2, 3];
    let row_hints = [3, 1, 3];

    let board: Board<X, Y> = Board::new(grid, col_hints, row_hints);

    _ = solve(&board);
}
