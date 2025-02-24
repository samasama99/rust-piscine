// You must create some functions for a tic-tac-toe checker.
//
// Create a function named tic_tac_toe,
//      which receives a tic-tac-toe table. It should return the appropriate string: "player O won", "player X won" or "tie".
//
// Also create the following functions, which each accept a player and a table.
//      These functions should return true if the player has completed one of the diagonals, rows or columns:

pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('O', table) || horizontal('O', table) || vertical('O', table) {
        "player O won".to_owned()
    } else if diagonals('X', table) || horizontal('X', table) || vertical('X', table) {
        "player X won".to_owned()
    } else {
        "tie".to_owned()
    }
}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    (table[0][0] == player && table[1][1] == player && table[2][2] == player)
        || (table[0][2] == player && table[1][1] == player && table[2][0] == player)
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    table
        .iter()
        .map(|h| h.iter().all(|&c| c == player))
        .any(|r| r)
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    (table[0][0] == player && table[1][0] == player && table[2][0] == player)
        || (table[0][1] == player && table[1][1] == player && table[2][1] == player)
        || (table[0][2] == player && table[1][2] == player && table[2][2] == player)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "tie",
            tic_tac_toe([['O', 'X', 'O'], ['O', 'P', 'X'], ['X', '#', 'X']])
        );
        assert_eq!(
            "player O won",
            tic_tac_toe([['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']])
        );

        let diag = [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']];

        assert_eq!("player X won", tic_tac_toe(diag));
    }
}
