use std::io::{stdin, stdout, BufRead, Write};

use crate::board::{Board, Player};

pub fn prompt_user(player: &Player, board: &Board) -> Result<usize, String> {
    let player = match player {
        Player::X => "X",
        Player::O => "O",
    };

    print!("\n{player}'s turn. Where to place? % ");
    stdout()
        .lock()
        .flush()
        .map_err(|_| "unable to flush from stdout".to_string())?;

    let mut buffer = String::new();
    stdin()
        .lock()
        .read_line(&mut buffer)
        .map_err(|_| "unable to read from stdin".to_string())?;

    println!();

    let buffer = buffer.trim();
    let idx = buffer
        .parse::<usize>()
        .map_err(|_| format!("'{buffer}' is not a valid position"))?;
    if !(0..9).contains(&idx) {
        return Err(format!("'{buffer}' is not a valid position"));
    }

    if board.occupied(idx) {
        return Err(format!("'{buffer}' is occupied"));
    }

    Ok(idx)
}
