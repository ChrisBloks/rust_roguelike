
mod character;
mod ui;
mod game_state;

use array2d::{Array2D, Error};
use crate::character::BaseCharacter;
use crate::location::Location;
use crate::game_state::GameState;
use crate::ui::user_interface::UserCommand;

fn main() {
    // Initialize game
    let mut g = initalize_game();

    // Loop game
    loop_game(&mut g);
}

fn initalize_game() -> GameState {
    // Generate base GameState
    let mut g = GameState {
        characters:  Vec::<BaseCharacter>::new(),
        terrain_map: GameState::initialize_map(None, None),
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
    while !g.terminate {
        // display game state
        g.render_map();
        
        // display user interface
        let user_input: String = ui::user_interface::generate_ui(&g.ui_state);

        // handle user input
        let command: Option<UserCommand> = ui::user_interface::interpret_input(user_input);

        // update game state
        g.handle_user_command(command);

        g.terminate = true;
    }
}