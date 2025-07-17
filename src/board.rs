use super::*;

use std::collections::HashMap;

const INDICES: [Index; 7] = [
    Index::A,
    Index::B,
    Index::C,
    Index::D,
    Index::E,
    Index::F,
    Index::G,
];

#[derive(Debug)]
pub struct Board {
    fields: HashMap<Indices, Field>,
    players: Vec<Player>,
    current_player: usize,
    highlight: Option<Indices>,
}

impl Board {
    pub fn init() -> Self {
        let mut board = HashMap::new();

        for i in &INDICES {
            for j in &INDICES {
                let indices = (i, j);
                let field = match indices {
                    (Index::D, Index::D) => continue,
                    (_, Index::D) => Field::Empty,
                    (Index::D, _) => Field::Empty,
                    (k, l) if k == l => Field::Empty,
                    (k, l) if (8 - *k as u8) == (*l as u8) => Field::Empty,
                    _ => continue,
                };
                board.insert((*i, *j), field); // empty
            }
        }

        Self {
            fields: board,
            players: vec![Player::new(&White), Player::new(&Black)],
            current_player: 0,
            highlight: None,
        }
    }

    pub fn modulo(&self) -> usize {
        self.current_player % 2
    }

    pub fn get(&self, index: Indices) -> Option<&Field> {
        self.fields.get(&index)
    }

    fn insert(&mut self, index: Indices, t: Token) {
        self.fields.insert(index, Field::Taken(t));
    }

    fn get_current_player(&self) -> &Player {
        let index = self.modulo();
        self.players.get(index).unwrap()
    }

    fn get_current_player_mut(&mut self) -> &mut Player {
        let index = self.modulo();
        self.players.get_mut(index).unwrap()
    }

    pub fn play(&mut self, index: Indices) {
        if let Some(field) = self.get(index) {
            // Phase 1: Placing Pieces
            if self.get_current_player().free_tokens() > 0 && field.empty() {
                self.place_piece(index);
                self.switch_player();
            }
        };
    }

    fn place_piece(&mut self, index: Indices) {
        let token = self.get_current_player_mut().use_token();
        self.insert(index, token);
        self.highlight = Some(index);
    }

    pub fn get_indices(x: u16, y: u16) -> Option<Indices> {
        let x1 = (x as f32 - 2.0) / 5.0;
        let x2 = x1.ceil();
        let x3 = x1 - x2;

        if y % 2 == 0 && x3 < -0.21 {
            Some((Index::from(x2 as u16), Index::from(y / 2)))
        } else {
            None
        }
    }

    fn switch_player(&mut self) {
        self.current_player += 1;
    }

    /// DRAWING

    fn draw_board(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Fg(Black))?;
        write!(f, "╋{}╋\n\r", "━".repeat(35))?;
        for _ in 0..=12 {
            write!(f, "┃{}┃\n\r", " ".repeat(35))?;
        }
        write!(f, "╋{}╋\n\r", "━".repeat(35))
    }

    fn draw_rect(&self, f: &mut std::fmt::Formatter<'_>, w: usize, h: usize) -> std::fmt::Result {
        let x = 18 - w / 2;
        let y = 8 - h / 2;

        write!(f, "{}", Fg(LightBlack))?;
        write!(f, "{}{}", goto(x + 1, y), "─".repeat(w))?;

        for i in 1..h - 1 {
            write!(f, "{}│{}│", goto(x, y + i), " ".repeat(w))?;
        }
        write!(f, "{}{}", goto(x + 1, h + y - 1), "─".repeat(w))
    }
}

fn goto(x: usize, y: usize) -> Goto {
    Goto(x as u16, y as u16)
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_fg(f)?;
        self.write_bg(f)?;
        self.draw_board(f)?;
        self.draw_rect(f, 29, 13)?;
        self.draw_rect(f, 19, 9)?;
        self.draw_rect(f, 9, 5)?;

        for y in &INDICES {
            for x in &INDICES {
                let indices = (*x, *y);

                if let Some(field) = self.get(indices) {
                    write!(f, "{}", Goto(*x as u16 * 5 - 1, *y as u16 * 2),)?;
                    write!(f, "{field}")?;
                    self.write_fg(f)?;
                    self.write_bg(f)?;
                }
            }
        }

        for (i, p) in self.players.iter().enumerate() {
            let current = if self.modulo() == i {
                format!("{}*", Fg(LightCyan))
            } else {
                format!("{} ", Reset)
            };
            write!(f, "{}{}{current} {}\n\r", Goto(2, 16 + i as u16), Reset, p)?;
        }
        Ok(())
    }
}

impl Color for Board {
    fn write_fg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(LightBlack.fg_str())
    }

    fn write_bg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(LightYellow.bg_str())
    }
}
