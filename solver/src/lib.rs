mod board;
mod direction;
mod helpers;
mod rail;

pub use board::Board;
use direction::Direction;
pub use rail::Rail;

type Point = (usize, usize);

#[inline(always)]
pub fn solve<const X: usize, const Y: usize>(
    mut board: Board<{ X }, { Y }>,
) -> Option<Board<{ X }, { Y }>> {
    if !board.adheres_hints() {
        return None;
    }
    let (cursor, direction) = board.find_start();
    let expected = board.rail_count();

    #[inline(always)]
    fn solve<const X: usize, const Y: usize>(
        board: &mut Board<{ X }, { Y }>,
        rail_count: u8,
        expected: u8,
        cursor: Point,
        direction: Direction,
    ) -> Option<Board<{ X }, { Y }>> {
        if !board.adheres_hints_at(cursor) {
            return None;
        }
        let offset = direction.offset();
        let out_of_bounds = cursor.0 == 0 && offset.0 < 0
            || cursor.1 == 0 && offset.1 < 0
            || cursor.0 == X - 1 && offset.0 > 0
            || cursor.1 == Y - 1 && offset.1 > 0;
        if out_of_bounds {
            if rail_count == expected {
                return Some(board.clone());
            } else {
                return None;
            }
        }
        // The usize cast won't panic as it's within bounds
        let x = (cursor.0 as isize + offset.0) as usize;
        let y = (cursor.1 as isize + offset.1) as usize;
        for rail in Rail::ALL {
            if !rail.points_to(direction) {
                continue;
            }
            if !board.can_place(rail, x, y) {
                continue;
            }
            // place rail, move cursor, change direction
            board.grid_mut()[y][x] = Some(rail);
            let new_cursor = (x, y);
            let new_direction = rail.points_other(!direction);
            // continue searching
            let solution = solve(board, rail_count + 1, expected, new_cursor, new_direction);
            // undo changes
            board.grid_mut()[y][x] = None;
            // did we find a solution?
            if solution.is_some() {
                return solution;
            }
        }
        None
    }
    solve(&mut board, 1, expected, cursor, direction)
}
