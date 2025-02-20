use board::{Board, Player};

mod board;
mod io;

const HUMAN: Player = Player::X;
const BOT: Player = HUMAN.opposite();

fn main() {
    let mut board = Board::new();
    let mut turn = HUMAN;
    loop {
        println!("{board}");
        if turn == HUMAN {
            let idx = match io::prompt_user(&turn, &board) {
                Ok(v) => v,
                Err(err) => {
                    println!("{err}");
                    continue;
                }
            };
            board = board.place_at(idx, &turn);
        } else {
            let position = board.minimax(&BOT, &turn);
            let (position, score) = match position {
                board::Minimaxxing::Result(_) => {
                    unreachable!("tried to move when game is over? {:018b}", board.as_u32())
                }
                board::Minimaxxing::Position(position, score) => (position, score),
            };
            println!("Bot wants to place at {position}, position has a score of {score}");
            board = board.place_at(position, &turn);
        }

        match board.status() {
            board::Status::Playing => turn = turn.opposite(),
            s @ (board::Status::Draw | board::Status::Winner(_)) => {
                match s {
                    board::Status::Winner(Player::X) => println!("X won!"),
                    board::Status::Winner(Player::O) => println!("O won!"),
                    board::Status::Draw => println!("Draw!"),
                    board::Status::Playing => unreachable!("matched previously"),
                };
                board = Board::new();
                turn = HUMAN;
            }
        }
    }
}
