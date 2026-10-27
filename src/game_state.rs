use crate::BaseCharacter;
use crate::Array2D;
use crate::Location;

pub enum Entity {
    WALL,
    CHARACTER,
    EMPTY,
}


pub struct GameState {
    pub characters: Vec<BaseCharacter>,
    pub terrain_map: Array2D<char>,
    pub terminate: bool,
    pub ui_state: String,
}

impl GameState {
    pub fn render_map(&self) {
        // Generate terrain map
        let mut display_map = self.terrain_map.clone();

        // place items (overwrite terrain display)

        // place characters (overwrite items display)
        self.characters.iter()
        .for_each(
            |char: &BaseCharacter| {
                let char_loc: &Location = &char.position;
                if let Some(loc) = display_map.get_mut(char_loc.y, char_loc.x) {
                    *loc = char.display;
                }
            }
        );

        // Display loop
        for row_iter in display_map.rows_iter() {
            for element in row_iter {
                print!("{} ", element);
            }
            println!();
        }
    }

    pub fn generate_map(max_x: Option<usize>, max_y: Option<usize>) -> Array2D<char> {
        // Default values
        let default_symbol: char = '_';
        let default_x = 6;
        let default_y = 8;

        let max_x: usize = max_x.unwrap_or(default_x);
        let max_y: usize = max_y.unwrap_or(default_y);

        // TODO: Create a terrain map
        let mut map = Array2D::filled_with(
            default_symbol, // Default terrain symbol
            max_y, // y-width
            max_x, // x-width
        );

        // Surround map with walls
        for x in 0..max_x {
            map.set(0, x, '#').expect("generate_map error in wall x-loop.");
            map.set(max_y-1, x, '#').expect("generate_map error in wall x-loop.");
        }

        for y in 0..max_y {
            map.set(y, 0, '#').expect("generate_map error in wall y-loop.");
            map.set(y, max_x-1, '#').expect("generate_map error in wall y-loop.");
        }

        map
    }
}