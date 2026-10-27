use crate::{GameState, location::Location};

pub enum MoveCommands {
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


    pub fn move_character(&mut self, direction: MoveCommands, g: &GameState) {

        // Calculate new location
        let new_loc: Location;
        match direction {
            MoveCommands::UP => new_loc = Location {x: self.position.x, y: self.position.y - 1},
            MoveCommands::RIGHT => new_loc = Location {x: self.position.x + 1, y: self.position.y},
            MoveCommands::DOWN => new_loc = Location {x: self.position.x, y: self.position.y + 1},
            MoveCommands::LEFT => new_loc = Location {x: self.position.x - 1, y: self.position.y},
        }

        // Check if new location is valid
    }
}
