use super::*;

use std::collections::HashMap;

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

        Position::valid_fields().iter().for_each(|pos| {
            fields.insert(*pos, Field::default());
        });

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

    pub fn play(&mut self, pos: &Position) -> Option<&dyn Color> {
        // Phase Interruption: player has to remove pieces
        if self.get_current_player().must_remove() {
            let color = self.get_current_player().color;

            if let Some(field) = self.get_field_mut(pos) {
                if !field.is_empty() {
                    if !field.belongs_to(color) {
                        field.vacate();
                        self.get_other_player_mut().increase_lost();
                        self.get_current_player_mut().change_remove(-1);
                    }
                }
                if !self.get_current_player().must_remove() {
                    self.next_turn();
                }
                // win condition, because of this ^^^
                if self.get_current_player_mut().lost() {
                    return Some(self.get_other_player_mut().color);
                }
            }
            return None;
        }

        let player = self.get_current_player();

        match player.hand_count() {
            // Phase 1: Placing Pieces
            1.. => self.place_piece(pos),
            _ => match player.points() {
                // Phase 2: Moving Pieces
                4.. => self.move_piece(pos),
                // Phase 3: Flying Pieces
                _ => self.fly_piece(pos),
            },
        }
        // self.highlight = Some(*pos.deref());

        None
    }

    fn place_piece(&mut self, pos: &Position) {
        match self.get_field_mut(pos) {
            // if clicked field is empty
            Some(field) if field.is_empty() => {
                // use the token and occupy the field
                self.get_current_player_mut().use_token();
                let token = Token(self.get_current_player().color);
                self.set_field(pos, Some(token));
                // check mill
                if !self.mill(pos) {
                    // and then give the staff to the next player
                    self.next_turn();
                }
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
            if self.filter_vacant(&neighbour).contains(&pos) {
                // swap them
                self.get_field_mut(&p).unwrap().vacate();
                self.place_piece(pos);
            }
            // and remove selection
            self.deselect_field();
        // Otherwise ...
        } else {
            if self
                .get_field(pos)
                .unwrap()
                .belongs_to(self.get_current_player().color)
            {
                let neighbour = pos.neighbours();
                // if it has an empty neighbour
                if !self.filter_vacant(&neighbour).is_empty() {
                    // select the requested field
                    self.select_field(pos);
                }
            }
        }
    }

    fn fly_piece(&mut self, pos: &Position) {
        // If there were selection...
        if let Some(selected_pos) = self.selected_field() {
            // ... and the new place is empty
            if self.get_field(pos).unwrap().is_empty() {
                if let Some(selected_field) = self.get_field_mut(&selected_pos) {
                    // swap them
                    selected_field.vacate();
                    self.place_piece(pos);
                }
                // and remove selection
                self.deselect_field();
            }

        // Otherwise ...
        } else {
            if self
                .get_field(pos)
                .unwrap()
                .belongs_to(self.get_current_player().color)
            {
                // select the requested field
                self.select_field(pos);
            }
        }
    }

    fn filter_vacant(&self, v: &Vec<Position>) -> Vec<Position> {
        v.iter()
            .filter_map(|p| match self.get_field(p) {
                Some(field) if field.is_empty() => Some(*p),
                _ => None,
            })
            .collect()
    }

    fn mill(&mut self, pos: &Position) -> bool {
        let color = self.get_current_player().color;

        let adjacent = self
            .fields
            .iter_mut()
            .filter(|(Position { x, y }, _)| match (pos.x, pos.y) {
                (..0, 0) if *y == 0 => *x < 0,
                (0.., 0) if *y == 0 => *x > 0,
                (0, ..0) if *x == 0 => *y < 0,
                (0, 0..) if *x == 0 => *y > 0,
                _ => pos.x == *x || pos.y == *y,
            })
            .filter_map(|(p, f)| match f.belongs_to(color) {
                true => Some(p),
                _ => None,
            })
            .collect::<Vec<_>>();

        let acc = adjacent.iter().fold(Position::new(0, 0), |p0, p| &p0 + *p);

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
                if x == 0 || y == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_current_player(board: &Board, color: &'static dyn Color, point: usize) {
        let player = board.get_current_player();
        assert_eq!(Token::from(player.color), Token::from(color));
        assert_eq!(player.points(), point);
    }

    fn assert_field_occupancy(board: &Board, pos: Position, color: Option<&'static dyn Color>) {
        let field = board.get_field(&pos).unwrap();
        match color {
            Some(c) => assert_eq!(field.vacancy, Some(Token(c))),
            _ => assert_eq!(field.vacancy, None),
        }
    }
    #[test]
    fn test_place_piece() {
        let mut board = Board::init();
        let fields = Position::valid_fields();
        let winner = board.play(&fields[5]);
        assert!(winner.is_none());
        assert_eq!(
            board.get_other_player_mut().to_string(),
            "White: points(1), hand_count(3), removes(0)"
        );
    }

    #[test]
    fn test_remove_piece() {
        let mut board = Board::init();
        let fields = Position::valid_fields();
        board.play(&fields[0]); // W (-3, 0)
        board.play(&fields[3]); // B (1, 0)
        board.play(&fields[1]); // W (-2, 0)
        board.play(&fields[4]); // B (2, 0)
        board.play(&fields[2]); // W (-1, 0)
        board.play(&fields[3]); // W (1, 0) - remove
        board.play(&fields[3]); // B (1, 0)
        board.play(&fields[6]); // W (0, -3) - run out
        assert!(board.play(&fields[8]).is_none()); // B (0, -1) - run out

        assert_current_player(&board, &White, 4);
        assert_eq!(
            board.get_other_player_mut().to_string(),
            "Black: points(3), hand_count(0), removes(0)"
        );
    }

    #[test]
    fn test_move_piece() {
        let mut board = Board::init();
        let fields = Position::valid_fields();
        // phase 1
        board.play(&fields[0]);
        board.play(&fields[1]);
        board.play(&fields[10]);
        board.play(&fields[11]);
        board.play(&fields[20]);
        board.play(&fields[21]);
        board.play(&fields[22]);
        board.play(&fields[23]);
        // phase 2
        assert_current_player(&board, &White, 4);
        board.play(&fields[1]); //  not yours
        assert_current_player(&board, &White, 4);
        board.play(&fields[2]); //  empty
        assert_current_player(&board, &White, 4);
        board.play(&fields[0]);
        board.play(&fields[12]); // too far
        assert_current_player(&board, &White, 4);
        board.play(&fields[9]); // no selected anymore
        assert_current_player(&board, &White, 4);
        board.play(&fields[0]); //  -1 -1 W
        board.play(&fields[9]); //   0 -1 W
        assert_current_player(&board, &Black, 4);
        assert_field_occupancy(&board, fields[0], None);
        assert_field_occupancy(&board, fields[9], Some(&White));
    }

    #[test]
    fn test_fly_piece() {
        let mut board = Board::init();
        let fields = Position::valid_fields();
        board.play(&fields[0]); // W (-3, 0)
        board.play(&fields[3]); // B (1, 0)
        board.play(&fields[1]); // W (-2, 0)
        board.play(&fields[4]); // B (2, 0)
        board.play(&fields[2]); // W (-1, 0)
        board.play(&fields[3]); // W (1, 0) - remove
        board.play(&fields[3]); // B (1, 0)
        board.play(&fields[6]); // W (0, -3) - run out
        assert!(board.play(&fields[8]).is_none()); // B (0, -1) - run out

        // phase 2
        assert_current_player(&board, &White, 4);
        board.play(&fields[3]); //  not yours
        assert_current_player(&board, &White, 4);
        board.play(&fields[0]);
        board.play(&fields[12]); //  no fly for you
        assert_current_player(&board, &White, 4);
        board.play(&fields[0]);
        board.play(&fields[12]); // too far
        assert_current_player(&board, &White, 4);
        board.play(&fields[9]); // no selected anymore
        assert_current_player(&board, &White, 4);
        board.play(&fields[0]);
        board.play(&fields[9]);
        // phase 3
        assert_current_player(&board, &Black, 3);
        board.play(&fields[3]);
        board.play(&fields[22]);
        assert_field_occupancy(&board, fields[3], None);
        assert_field_occupancy(&board, fields[22], Some(&Black));
    }
}
