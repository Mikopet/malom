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

    fn remove(&mut self, index: Indices) {
        self.fields.insert(index, Field::Empty);
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
        // force remove action
        if let Some(Field::Taken(t)) = self.get(index) {
            if self.get_current_player().get_remove() > 0 {
                if self.get_current_player().token != *t {
                    self.remove_piece(index);
                    return;
                }
            }
        }
        if let Some(field) = self.get(index) {
            // Phase 1: Placing Pieces
            if self.get_current_player().free_tokens() > 0 && field.empty() {
                self.place_piece(index);
                self.get_current_player_mut().add_point();
            } else if self.get_current_player().free_tokens() == 0 {
                self.move_piece(index);
            } else if self.get_current_player().points() == 3 {
                self.flying_piece(index);
            }
        };
    }
    fn move_piece(&mut self, i: Indices) {
        let mut player = self.get_current_player_mut();
        if let Some(v) = player.selected {
            let neighbour = player.neighbour(v);
            let filtered_neighbour = self.filter_empty(neighbour);
            println!(" Akár ide is mozgathattál volna: {:?} ", filtered_neighbour);
            let mut found = false;
            for num in &filtered_neighbour {
                if num == &i {
                    found = true;
                    break;
                }
            }
            if found {
                self.remove(v);
                let token = self.get_current_player().token;
                self.insert(i, token);
                if !self.mill(i) {
                    self.switch_player();
                }
            } else {
            }
        } else {
            if let Some(selected_token) = self.get(i) {
                match selected_token {
                    Field::Taken(token) => {
                        let mut player: &mut Player;
                        if *token == self.get_current_player().token {
                            self.get_current_player_mut().set_selected(i);
                            println!("Ezt mozgatod: {:?} Válassz egy célt!", i);
                        } else {
                            println!("Saját bábut válasszál!");
                        }
                    }
                    Field::Empty => {
                        println!("Válassz egy saját színű bábút!");
                    }
                }
            }
        }
    }
    /*
        fn move_piece(&mut self, i: Indices) {
        //let neighbour = self.neighbour(self.get_current_player().selected.unwrap());
        let mut player = self.get_current_player_mut();
        if let Some(v) = player.selected {
            let neighbour = player.neighbour();
            if self.filter_empty(neighbour).contains(&v) {
                self.remove_piece(v);
                self.place_piece(i);
            }
        } else {
            player.set_selected(i);
            let neighbour = player.neighbour();
            if self.filter_empty(neighbour).len() == 0 {
                player.selected = None;
            }
        }
    }
    */

    fn filter_empty(&self, v: Vec<Indices>) -> Vec<Indices> {
        let mut q = Vec::new();
        for (i, f) in &self.fields {
            if let Field::Empty = f {
                q.push(*i);
            }
        }
        q
    }
    fn flying_piece(&self, i: Indices) {}
    fn place_piece(&mut self, i: Indices) {
        let token = self.get_current_player_mut().use_token();
        self.insert(i, token);
        self.get_current_player_mut().add_point();
        self.highlight = Some(i);
        if !self.mill(i) {
            self.switch_player();
        }
    }

    fn remove_piece(&mut self, i: Indices) {
        // let token = self.get_current_player_mut().use_token();
        self.remove(i);
        self.get_current_player_mut().remove_remove();
        self.highlight = Some(i);
        if self.get_current_player().get_remove() == 0 {
            self.switch_player();
        }
    }
    fn mill(&mut self, i: Indices) -> bool {
        let token = self.get_current_player().token;

        let adjacent = self
            .fields
            .iter_mut()
            .filter(|((x, y), _)| match (i.0 as u8, i.1 as u8) {
                (..4, 4) if *y == Index::D => *x < Index::D,
                (4.., 4) if *y == Index::D => *x > Index::D,
                (4, ..4) if *x == Index::D => *y < Index::D,
                (4, 4..) if *x == Index::D => *y > Index::D,
                _ => i.0 == *x || i.1 == *y,
            })
            .filter_map(|(i, f)| match f {
                Field::Taken(t) if *t == token => Some(i),
                _ => None,
            })
            .collect::<Vec<_>>();

        let acc = adjacent
            .iter()
            .fold((0, 0), |a, i| (a.0 + i.0 as u8, a.1 + i.1 as u8));
        println!("{:?}", &acc);

        match adjacent.len() {
            5 => {
                self.get_current_player_mut().add_remove(2);
                true
            }
            4 => {
                self.get_current_player_mut().add_remove(1);
                true
            }
            3 => match acc {
                (x, y) if x == 12 || y == 12 => {
                    self.get_current_player_mut().add_remove(1);
                    true
                }
                _ => false,
            },
            n => {
                dbg!(n);
                false
            }
        }
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
        let x = 19 - w / 2;
        let y = 8 - h / 2;

        write!(f, "{}", Fg(LightBlack))?;
        write!(f, "{}{}", goto(x, y), "─".repeat(w))?;

        for i in 1..h {
            write!(f, "{}│", goto(x, y + i))?;
            write!(f, "{}│", goto(x + w, y + i))?;
        }
        write!(f, "{}{}", goto(x, h + y), "─".repeat(w))
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
        self.draw_rect(f, 30, 12)?;
        self.draw_rect(f, 20, 8)?;
        self.draw_rect(f, 10, 4)?;

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
