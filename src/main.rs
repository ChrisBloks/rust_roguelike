
mod character;
mod location;
mod ui;
mod game_state;

use array2d::{Array2D, Error};
use crate::character::BaseCharacter;
use crate::location::Location;
use crate::game_state::GameState;

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
        ui_state: String::from("map"),
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
    g.render_map();
    
    // display user interface
    let user_input: String = ui::user_interface::generate_ui(&g.ui_state);

    // handle user input
    let command: 


    // update game state
    g.terminate = true;

}