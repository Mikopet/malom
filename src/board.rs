use super::*;

use std::collections::HashMap;

// TODO: make board size configurable (Config struct)
// const BOARD_SIZE: usize = 3;

// TODO: change coordinate system
pub const BOARD_RANGE: std::ops::Range<isize> = 1..8;
// pub const BOARD_RANGE: std::ops::Range<isize> =
//     0 - (BOARD_SIZE as isize)..1 + (BOARD_SIZE as isize);

#[derive(Debug)]
pub struct Board {
    fields: HashMap<Position, Field>,
    players: Vec<Player>,
    turn_count: usize,
    selected: Option<Position>,
    // highlight: Option<Position>,
}

impl Board {
    pub fn init() -> Self {
        let mut fields = HashMap::with_capacity(24);

        for y in BOARD_RANGE {
            for x in BOARD_RANGE {
                match (x, y) {
                    (4, 4) => continue,
                    (_, 4) => {}
                    (4, _) => {}
                    (k, l) if k == l => {}
                    (k, l) if (8 - k) == l => {}
                    _ => continue,
                };
                fields.insert(Position::new(x, y), Field::default());
            }
        }

        Self {
            fields,
            players: vec![Player::new(&White), Player::new(&Black)],
            turn_count: 0,
            selected: None,
            // highlight: None,
        }
    }

    // Field related

    pub fn get_field(&self, pos: &Position) -> Option<&Field> {
        self.fields.get(pos)
    }

    pub fn get_field_mut(&mut self, pos: &Position) -> Option<&mut Field> {
        self.fields.get_mut(pos)
    }

    fn set_field(&mut self, pos: &Position, token: Option<Token>) -> Option<&Field> {
        if let Some(field) = self.fields.get_mut(pos) {
            match token {
                None => Some(field.vacate()),
                Some(t) => Some(field.occupy(t)),
            }
        } else {
            None
        }
    }

    fn selected_field(&self) -> Option<Position> {
        self.selected
    }
    fn select_field(&mut self, pos: &Position) {
        self.selected = Some(*pos);
    }

    fn deselect_field(&mut self) {
        self.selected = None;
    }

    // Player related

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn get_current_player_index(&self) -> usize {
        self.turn_count % self.players.len()
    }

    fn get_other_player_index(&self) -> usize {
        (self.turn_count + 1) % self.players.len()
    }

    fn next_turn(&mut self) {
        self.turn_count += 1;
    }

    fn get_current_player(&self) -> &Player {
        let index = self.get_current_player_index();
        self.players.get(index).unwrap()
    }

    fn get_current_player_mut(&mut self) -> &mut Player {
        let index = self.get_current_player_index();
        self.players.get_mut(index).unwrap()
    }

    fn get_other_player_mut(&mut self) -> &mut Player {
        let index = self.get_other_player_index();
        self.players.get_mut(index).unwrap()
    }

    // Board related

    pub fn play(&mut self, pos: &Position) {
        // Phase Interruption: player has to remove pieces
        if self.get_current_player().must_remove() {
            let color = self.get_current_player().color;
            if let Some(field) = self.get_field_mut(pos) {
                if !field.belongs_to(color) {
                    field.vacate();
                    self.get_other_player_mut().increase_lost();
                    self.get_current_player_mut().change_remove(-1);
                }
                if !self.get_current_player().must_remove() {
                    self.next_turn();
                }
            }
        }

        let player = self.get_current_player();

        match player.hand_count() {
            // Phase 1: Placing Pieces
            1.. => self.place_piece(pos),
            _ => match player.points() {
                // Phase 2: Moving Pieces
                3.. => self.move_piece(pos),
                // Phase 3: Flying Pieces
                _ => self.fly_piece(pos),
            },
        }
        // self.highlight = Some(*pos.deref());

        // TODO: probably if it's mill, turn count should be decrease
        if !self.mill(pos) {
            // self.previous_turn();
        }
    }

    fn place_piece(&mut self, pos: &Position) {
        match self.get_field_mut(pos) {
            // if clicked field is empty
            Some(field) if field.is_empty() => {
                // use the token and occupy the field
                let token = self.get_current_player_mut().use_token();
                self.set_field(pos, token);
                // and then give the staff to the next player
                self.next_turn();
            }
            _ => {}
        };
    }

    fn move_piece(&mut self, pos: &Position) {
        let selected_field = self.selected_field();

        // If there were selection...
        if let Some(p) = selected_field {
            let neighbour = p.neighbours();
            // ... and it was empty
            if self.filter_empty(&neighbour).contains(&pos) {
                // swap them
                self.get_field_mut(&p).unwrap().vacate();
                self.place_piece(pos);
                // and remove selection
                self.deselect_field();
            }
        // Otherwise ...
        } else {
            let neighbour = pos.neighbours();
            // if it has an empty neighbour
            if self.filter_empty(&neighbour).len() > 0 {
                // select it
                self.select_field(pos);
            }
        }
    }

    fn fly_piece(&self, _pos: &Position) {}

    fn filter_empty(&self, v: &Vec<Position>) -> Vec<Position> {
        v.iter()
            .filter_map(|p| match self.get_field(p) {
                None => Some(*p),
                Some(_) => None,
            })
            .collect()
    }

    fn mill(&mut self, pos: &Position) -> bool {
        let color = self.get_current_player().color;

        let adjacent = self
            .fields
            .iter_mut()
            .filter(|(Position { x, y }, _)| match (pos.x, pos.y) {
                (..4, 4) if *y == 4 => *x < 4,
                (4.., 4) if *y == 4 => *x > 4,
                (4, ..4) if *x == 4 => *y < 4,
                (4, 4..) if *x == 4 => *y > 4,
                _ => pos.x == *x || pos.y == *y,
            })
            .filter_map(|(p, f)| match f.belongs_to(color) {
                true => Some(p),
                _ => None,
            })
            .collect::<Vec<_>>();

        let acc = adjacent.iter().fold(Position::new(0, 0), |p0, p| &p0 + *p);
        println!("{:?}", &acc);

        match adjacent.len() {
            5 => {
                self.get_current_player_mut().change_remove(2);
                true
            }
            4 => {
                self.get_current_player_mut().change_remove(1);
                true
            }
            3 => {
                let Position { x, y } = acc;
                if x == 12 || y == 12 {
                    self.get_current_player_mut().change_remove(1);
                    true
                } else {
                    false
                }
            }

            _ => false,
        }
    }
}
