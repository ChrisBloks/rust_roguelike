
mod character;
mod ui;
mod game_state;
mod location;
mod entity;

use array2d::{Array2D, Error};
use crate::character::BaseCharacter;
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
    let mut g = GameState::new();

    // Add Player
    let player_character: Entity = Entity::new
    
    BaseCharacter {
        name: String::from("Marius"),
        position: (2, 1), // Default start location
        display: '@',
    };
    g.add_entity(PlayerCharacter(player_character));

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