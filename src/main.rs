#[allow(dead_code)]

fn refresh_board(board: &Vec<Vec<bool>>) {
    print!("{}[2J", 27 as char); // clean screen
    board.iter().for_each(|row| {
        print!("|");
        row.iter().for_each(|col| {
            let ch = match col {
                true => "█",
                false => " ",
            };
            print!("{}", ch);
        });
        print!("|\n");
    });
}

fn step_board(board: &mut Vec<Vec<bool>>) {
    let mut next_state = board.clone();
    for x in 0..board.len() {
        for y in 0..board[0].len() {
            let live_neighbour_count = count_live_neighbours(board, x, y);
            let is_alive = board[x][y];
            next_state[x][y] = match (is_alive, live_neighbour_count) {
                (true, x) if x < 2 => false,
                (true, x) if x > 3 => false,
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                (otherwise, _) => otherwise,
            };
        }
    }
    *board = next_state;
}

fn count_live_neighbours(board: &Vec<Vec<bool>>, x: usize, y: usize) -> u8 {
    let mut count = 0;
    let board_width = board.len() - 1;
    let board_height = board[0].len() - 1;

    if x > 0 && y > 0 && board[x - 1][y - 1] {
        count += 1;
    }
    if x > 0 && board[x - 1][y] {
        count += 1;
    }
    if x > 0 && y < board_height && board[x - 1][y + 1] {
        count += 1;
    }
    if y > 0 && board[x][y - 1] {
        count += 1;
    }
    if y < board_height && board[x][y + 1] {
        count += 1;
    }
    if y > 0 && x < board_width && board[x + 1][y - 1] {
        count += 1;
    }
    if x < board_width && board[x + 1][y] {
        count += 1;
    }
    if x < board_width && y < board_height && board[x + 1][y + 1] {
        count += 1;
    }
    
    count
}

fn main() {
    let mut board = vec![vec![false; 12]; 12];

    board[0][1] = true;
    board[1][2] = true;
    board[2][0] = true;
    board[2][1] = true;
    board[2][2] = true;

    loop {
        refresh_board(&board);
        step_board(&mut board);
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_live_neighbours_inner_board() {
        let board = vec![vec![true; 3]; 3];
        assert_eq!(count_live_neighbours(&board, 1, 1), 8);
    }

    #[test]
    fn count_live_neighbours_board_edges() {
        let board = vec![vec![true; 3]; 3];
        assert_eq!(count_live_neighbours(&board, 0, 0), 3);
        assert_eq!(count_live_neighbours(&board, 0, 1), 5);
        assert_eq!(count_live_neighbours(&board, 0, 2), 3);
        assert_eq!(count_live_neighbours(&board, 1, 0), 5);
        assert_eq!(count_live_neighbours(&board, 1, 1), 8);
        assert_eq!(count_live_neighbours(&board, 1, 2), 5);
        assert_eq!(count_live_neighbours(&board, 2, 0), 3);
        assert_eq!(count_live_neighbours(&board, 2, 1), 5);
        assert_eq!(count_live_neighbours(&board, 2, 2), 3);
    }

    #[test]
    fn any_live_cell_with_fewer_than_two_live_neighbours_dies() {
        let mut board = vec![vec![false; 3]; 3];
        board[1][1] = true;
        board[0][1] = true;
        step_board(&mut board);
        assert_eq!(board[1][1], false);
    }

    #[test]
    fn any_live_cell_with_two_or_three_live_neighbours_lives_on_to_the_next_generation() {
        let mut board = vec![vec![false; 3]; 3];
        board[1][1] = true;
        board[0][1] = true;
        board[0][2] = true;
        board[1][2] = true;
        step_board(&mut board);
        assert_eq!(board[1][1], true);
    }

    #[test]
    fn any_live_cell_with_more_than_three_live_neighbours_dies() {
        let mut board = vec![vec![false; 3]; 3];
        board[1][1] = true;
        board[0][1] = true;
        board[0][2] = true;
        board[1][2] = true;
        board[2][2] = true;
        step_board(&mut board);
        assert_eq!(board[1][1], false);
    }

    #[test]
    fn any_dead_cell_with_exactly_three_live_neighbours_becomes_a_live_cell() {
        let mut board = vec![vec![false; 3]; 3];
        board[0][1] = true;
        board[0][2] = true;
        board[1][2] = true;
        step_board(&mut board);
        assert_eq!(board[1][1], true);
    }
}
