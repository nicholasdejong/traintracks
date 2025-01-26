use solver::{solve, Board, Rail::*};

fn main() {
    const X: usize = 12;
    const Y: usize = 11;

    let row_hints = [4, 8, 4, 7, 6, 6, 6, 8, 7, 3, 2];
    let col_hints = [4, 4, 4, 5, 6, 5, 6, 5, 3, 6, 5, 8];

    let mut grid = [[None; X]; Y];
    grid[0][4] = Some(SW);
    grid[0][10] = Some(SW);
    grid[1][0] = Some(WE);
    grid[2][4] = Some(NE);
    grid[4][6] = Some(NE);
    grid[4][10] = Some(NW);
    grid[5][0] = Some(SW);
    grid[5][3] = Some(SE);
    grid[8][5] = Some(SE);
    grid[8][7] = Some(SE);
    grid[9][6] = Some(SW);

    let board: Board<X, Y> = Board::new(grid, col_hints, row_hints);

    _ = solve(&board);
}
