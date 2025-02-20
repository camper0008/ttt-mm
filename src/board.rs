use std::fmt::Display;

#[repr(transparent)]
pub struct Board(u32);

pub enum Player {
    X,
    O,
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

    pub const fn occupied(&self, idx: usize) -> bool {
        let ones = (self.0 & Self::ACCESS[idx]).count_ones();
        ones != 1
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

    pub const fn place_at(self, idx: usize, player: &Player) -> Self {
        let pattern = Self::ACCESS[idx];
        Self(
            (self.0 & (!pattern))
                | match player {
                    Player::O => 0,
                    Player::X => pattern,
                },
        )
    }

    pub const fn winner(&self) -> Option<Player> {
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
