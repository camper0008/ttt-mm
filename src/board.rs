use std::fmt::Display;

#[repr(transparent)]
pub struct Board(u32);

#[derive(PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub const fn opposite(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

pub enum Minimaxxing {
    Result(i8),
    Position(usize, i8),
}

pub enum Status {
    Playing,
    Winner(Player),
    Draw,
}

impl Board {
    pub const ACCESS: [u32; 9] = [
        0b110000_000000_000000,
        0b001100_000000_000000,
        0b000011_000000_000000,
        0b000000_110000_000000,
        0b000000_001100_000000,
        0b000000_000011_000000,
        0b000000_000000_110000,
        0b000000_000000_001100,
        0b000000_000000_000011,
    ];

    pub const fn new() -> Self {
        Board(0b01_01_01_01_01_01_01_01_01)
    }

    pub const fn indexes_available(&self) -> [bool; 9] {
        let mut result = [false; 9];
        let mut idx = 0;
        loop {
            result[idx] = (self.0 & Self::ACCESS[idx]).count_ones() == 1;
            idx += 1;
            if idx == 9 {
                break result;
            }
        }
    }

    pub const fn occupied(&self, idx: usize) -> bool {
        let ones = (self.0 & Self::ACCESS[idx]).count_ones();
        ones != 1
    }

    pub const fn as_u32(&self) -> u32 {
        self.0
    }

    pub const fn value_at(&self, idx: usize) -> Option<Player> {
        let ones = (self.0 & Self::ACCESS[idx]).count_ones();
        if ones == 0 {
            Some(Player::O)
        } else if ones == 1 {
            None
        } else if ones == 2 {
            Some(Player::X)
        } else {
            None
        }
    }

    pub const fn place_at(&self, idx: usize, player: &Player) -> Self {
        let pattern = Self::ACCESS[idx];
        Self(
            (self.0 & (!pattern))
                | match player {
                    Player::O => 0,
                    Player::X => pattern,
                },
        )
    }

    const fn game_over(&self) -> bool {
        let mut i = 0;
        loop {
            if (self.0 >> i) & 0b11 == 0b01 {
                break false;
            }
            i += 2;
            if i > 18 {
                break true;
            }
        }
    }

    pub const fn status(&self) -> Status {
        if let Some(winner) = self.winner() {
            Status::Winner(winner)
        } else {
            if self.game_over() {
                Status::Draw
            } else {
                Status::Playing
            }
        }
    }

    const fn winner(&self) -> Option<Player> {
        let winning_patterns = [
            0b111111_000000_000000,
            0b000000_111111_000000,
            0b000000_000000_111111,
            0b110000_110000_110000,
            0b001100_001100_001100,
            0b000011_000011_000011,
            0b000011_000011_000011,
            0b110000_001100_000011,
            0b000011_001100_110000,
        ];

        let mut idx = 0;
        loop {
            let pattern = winning_patterns[idx];
            let ones = (self.0 & pattern).count_ones();
            if ones == 6 {
                break Some(Player::X);
            } else if ones == 0 {
                break Some(Player::O);
            }
            idx += 1;
            if idx == winning_patterns.len() {
                break None;
            }
        }
    }

    pub fn minimax(&self, maximizer: &Player, turn: &Player) -> Minimaxxing {
        match self.status() {
            Status::Winner(winner) => {
                return Minimaxxing::Result(Self::evaluate_winner_pts(maximizer, &winner));
            }
            Status::Draw => return Minimaxxing::Result(0),
            Status::Playing => {}
        }

        let children = self
            .indexes_available()
            .into_iter()
            .enumerate()
            .filter_map(|(pos, available)| if available { Some(pos) } else { None })
            .map(|pos| (pos, self.place_at(pos, turn)))
            .map(|(pos, board)| (pos, board.minimax(&maximizer, &turn.opposite())))
            .map(|(pos, negamaxx)| match negamaxx {
                Minimaxxing::Position(_, v) => (pos, v.clamp(-1, 1)),
                Minimaxxing::Result(v) => (pos, v * 2),
            });

        let chosen = if turn == maximizer {
            children.max_by(|(_, left_score), (_, right_score)| left_score.cmp(&right_score))
        } else {
            children.min_by(|(_, left_score), (_, right_score)| left_score.cmp(&right_score))
        };

        chosen
            .map(|(pos, score)| Minimaxxing::Position(pos, score))
            .expect("game is not over")
    }

    fn evaluate_winner_pts(maximizer: &Player, winner: &Player) -> i8 {
        if maximizer == winner {
            1
        } else {
            -1
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rows: [_; 3] = std::array::from_fn(|row| {
            let row: [_; 3] = std::array::from_fn(|position| {
                let position = row * 3 + position;
                match self.value_at(position) {
                    Some(Player::O) => "\x1b[1;33mO\x1b[0m".to_string(),
                    Some(Player::X) => "\x1b[1;31mX\x1b[0m".to_string(),
                    None => format!("\x1b[0;37m{position}\x1b[0m"),
                }
            });
            format!("| {} |", row.join(" | "))
        });

        let rows = format!(
            "+---+---+---+\n{}\n+---+---+---+",
            rows.join("\n+---+---+---+\n")
        );

        write!(f, "{rows}")
    }
}

#[cfg(test)]
mod test {
    use super::Board;

    #[test]
    fn game_over() {
        let board = Board(0b110100011111110000);
        assert!(!board.game_over())
    }
}
