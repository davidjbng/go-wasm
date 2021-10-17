pub struct Board {
    pub positions: [[State; 19]; 19],
}

impl Board {
    pub fn empty() -> Board {
        Self {
            positions: Default::default(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum State {
    Empty,
    White,
    Black,
}

impl Default for State {
    fn default() -> Self {
        State::Empty
    }
}
