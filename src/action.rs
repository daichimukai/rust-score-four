#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Action(u8);

impl Action {
    pub fn new(pos: u8) -> Self {
        Action(pos)
    }

    pub fn position(&self) -> u8 {
        self.0
    }
}
