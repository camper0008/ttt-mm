use board::{Board, Player};

mod board;
mod io;

fn main() {
    let mut board = Board::new();
    let mut turn = Player::X;
    loop {
        println!("{board}");
        let idx = match io::prompt_user(&turn, &board) {
            Ok(v) => v,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };
        board = board.place_at(idx, &turn);
        if let Some(winner) = board.winner() {
            let winner = match winner {
                Player::X => "X",
                Player::O => "O",
            };
            println!("{winner} won!");
            board = Board::new();
            turn = Player::X;
            continue;
        }
        turn = match turn {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
