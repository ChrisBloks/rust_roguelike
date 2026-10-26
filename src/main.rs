
mod character;
mod location;

use array2d::{Array2D, Error};
use crate::character::BaseCharacter;
use crate::location::Location;

struct GameState {
    characters: Vec<BaseCharacter>,
    terrain_map: Array2D<char>,
    terminate: bool,
}

impl GameState {
    fn render(&self) {
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

    fn generate_map(max_x: Option<usize>, max_y: Option<usize>) -> Array2D<char> {
        // Default values
        let default_symbol: char = '0';
        let default_x = 6;
        let default_y = 8;

        let max_x: usize = max_x.unwrap_or(default_x);
        let max_y: usize = max_y.unwrap_or(default_y);

        // TODO: Create a terrain map
        let mut map = Array2D::filled_with(
            '0', // Default terrain symbol
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

fn main() {
    // Initialize game
    let mut g = initalize_game();

    // Loop game
    while !g.terminate {
        loop_game(&mut g);
    }

}

fn initalize_game() -> GameState {
    // Generate base GameState
    let mut g = GameState {
        characters:  Vec::<BaseCharacter>::new(),
        terrain_map: GameState::generate_map(None, None),
        terminate: false,
    };

    // Add Player
    let player_character = BaseCharacter {
        name: String::from("Marius"),
        position: Location {x: 2, y: 1},
        display: '@',
    };
    g.characters.push(player_character);

    // Add ???



    g
}


fn loop_game(g: &mut GameState) {
    // display game state
    g.render();
    
    // display user interface

    // handle user input

    // update game state
    g.terminate = true;

}