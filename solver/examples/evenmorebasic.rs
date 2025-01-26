use solver::{Board, Rail::*};

fn main() {
    const X: usize = 2;
    const Y: usize = 2;

    let grid = [[Some(SE), Some(WE)], [Some(NE), Some(WE)]];

    let row_hints = [2, 2];
    let col_hints = [2, 2];

    let board: Board<X, Y> = Board::new(grid, col_hints, row_hints);

    let (x, y) = (1, 1);
    let rail = WE;
    dbg!(board.can_place(rail, x, y));
}
