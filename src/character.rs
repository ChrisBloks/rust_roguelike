
use crate::{GameState, location::Location};

pub enum MoveCommand {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
pub struct BaseCharacter {
    pub name: String,
    pub position: Location,
    pub display: char,
}

impl BaseCharacter {


    pub fn move_character(&mut self, direction: MoveCommand, g: &GameState) {

        // Calculate new location
        let new_loc: (usize, usize);
        match direction {
            MoveCommand::UP => new_loc = (self.position.x, self.position.y - 1),
            MoveCommand::RIGHT => new_loc = (self.position.x + 1, self.position.y),
            MoveCommand::DOWN => new_loc = (self.position.x, self.position.y + 1),
            MoveCommand::LEFT => new_loc = (self.position.x -1 , self.position.y),
        }

        // Check if new location is valid
    }
}
