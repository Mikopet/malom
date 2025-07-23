use crate::*;

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // board
        self.write_fg(f)?;
        self.write_bg(f)?;
        self.draw_board(f)?;
        self.draw_rect(f, 30, 12)?;
        self.draw_rect(f, 20, 8)?;
        self.draw_rect(f, 10, 4)?;
        self.draw_cross(f)?;

        // tokens
        #[allow(unused_must_use)]
        Position::valid_fields().iter().for_each(|pos| {
            write!(f, "{pos}{}", self.get_field(&pos).unwrap());
            self.write_fg(f);
            self.write_bg(f);
        });

        // selection
        if let Some(pos) = self.selected_field() {
            write!(f, "{pos}{}☉︎︎", Fg(Cyan))?;
        }

        // highlight
        if let Some(pos) = self.highlight {
            write!(f, "{pos}{}{}«", Left(1), Fg(LightGreen))?;
            write!(f, "{pos}{}{}»", Right(1), Fg(LightGreen))?;
        }

        // dashboard
        for (index, player) in self.players().iter().enumerate() {
            let current = if self.get_current_player_index() == index {
                format!("{}*", Fg(LightCyan))
            } else {
                format!("{} ", Reset)
            };
            let index = 16u16 + index as u16;
            write!(f, "{}{}{current} {}\n\r", Goto(2, index), Reset, player)?;
        }
        Ok(())
    }
}

impl Board {
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

        self.write_fg(f)?;
        write!(f, "{}{}", goto(x, y), "─".repeat(w))?;

        for i in 1..h {
            write!(f, "{}│", goto(x, y + i))?;
            write!(f, "{}│", goto(x + w, y + i))?;
        }
        write!(f, "{}{}", goto(x, h + y), "─".repeat(w))
    }

    fn draw_cross(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_fg(f)?;
        write!(f, "{}{}", goto(5, 8), "─".repeat(29))?;
        for i in 3..15 {
            write!(f, "{}│", goto(19, i))?;
        }
        write!(f, "{}{}", goto(15, 7), " ".repeat(9))?;
        write!(f, "{}{}", goto(15, 8), " ".repeat(9))?;
        write!(f, "{}{}", goto(15, 9), " ".repeat(9))
    }
}

fn goto(x: usize, y: usize) -> Goto {
    Goto(x as u16, y as u16)
}

impl Color for Board {
    fn write_fg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Yellow.fg_str())
    }

    fn write_bg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(LightYellow.bg_str())
    }
}
