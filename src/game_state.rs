use crate::Array2D;
use crate::BaseCharacter;
use crate::game_state::entity::EntityType;
use crate::ui::user_interface::UserCommand;

pub mod location {
    

    use crate::game_state::entity::{Entity, EntityType};

    pub struct Location<'a> {
        pub x: usize,
        pub y: usize,
        entity: Option<&'a Entity>,
    }

    impl Location<'_> {
        pub fn is_traversable(&self) -> Result<bool, &str> {
            if let Some(e) = &self.entity {
                match e.e_type {
                    EntityType::Wall => Ok(false),
                    _ => Ok(true),
                }
            } else {
                Err("Could not determine the traversability of {self.x}, {self.y}")
            }
        }

        pub fn get_entity(&self) -> Option<&Entity> {
            self.entity
        }

        pub fn create_wall(x: usize, y:usize, wall_entity: &Entity) ->  Location {
            Location { 
                x,
                y,
                entity: Some(wall_entity) 
            }
        } 
    }
}

mod entity {

    use crate::character::BaseCharacter;

    pub enum EntityType {
        Wall,
        PlayerCharacter(BaseCharacter),
        NPC(BaseCharacter),
        Empty,
        Other,
    }

    pub struct Entity {
        pub e_type: EntityType,
        location: (usize, usize),
        display: Option<char>,
    }

    impl Entity {
        /// Get an Entity's char representation
        pub fn render(&self) -> char {
            match self.e_type {
                EntityType::PlayerCharacter(_) | EntityType::NPC(_) => {
                    self.display.clone().unwrap_or('%')
                }
                EntityType::Empty => '_',
                EntityType::Wall => '#',
                EntityType::Other => '?',
            }
        }

        /// Get an entity's location as (x,y) coordinates
        pub fn get_location(&self) -> (usize, usize) {
            self.location.clone()
        }

        pub fn set_location(&mut self, x: usize, y: usize) {
            self.location = (x, y);
        }
    }
}


use crate::game_state::location::Location;

pub struct GameState<'a> {
    pub characters: Vec<BaseCharacter>,
    pub location_map: Array2D<Location<'a>>,
    pub terminate: bool,
    pub ui_state: String,
}

impl GameState<'_> {
    /// Display the current map state to the user
    pub fn render_map(&self) {
        // Generate terrain map
        let mut display_map = self.terrain_map.clone();

        // place items (overwrite terrain display)

        // place characters (overwrite items display)
        self.characters
            .iter()
            .for_each(|character: &BaseCharacter| {
                let (x, y): &(usize, usize) = &character.position;
                if let Some(loc_char) = display_map.get_mut(*y, *x) {
                    *loc_char = character.display;
                }
            });

        // Display loop
        for row_iter in display_map.rows_iter() {
            for element in row_iter {
                print!("{} ", element);
            }
            println!();
        }
    }

    ///
    pub fn initialize_map(max_x: Option<usize>, max_y: Option<usize>) -> Array2D<Location> {
        // Default values
        let default_symbol: char = '_';
        let default_x = 6;
        let default_y = 8;

        let max_x: usize = max_x.unwrap_or(default_x);
        let max_y: usize = max_y.unwrap_or(default_y);

        let mut map = Array2D::filled_with(
            default_symbol, // Default terrain symbol
            max_y,          // y-width
            max_x,          // x-width
        );

        // Surround map with walls
        for x in 0..max_x {
            map.set(0, x, '#')
                .expect("generate_map error in wall x-loop.");
            map.set(max_y - 1, x, '#')
                .expect("generate_map error in wall x-loop.");
        }

        for y in 0..max_y {
            map.set(y, 0, '#')
                .expect("generate_map error in wall y-loop.");
            map.set(y, max_x - 1, '#')
                .expect("generate_map error in wall y-loop.");
        }

        map
    }

    pub fn handle_user_command(&self, cmd: Option<UserCommand>) {
        // validate cmd

        // execute cmd (update gamestate)
    }

    fn valid_move_to(&self, loc: Location) -> bool {
        true
    }
}
