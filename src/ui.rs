/// Module responsible for displaying the UI and interpreting user inputs
pub mod user_interface {

    enum UserCommand {
        QUIT,
        MOVE(MoveCommand),
    }

    use std::io;
    use crate::{character::MoveCommand};

    // ==================================================================
    // Ui-generation
    // ==================================================================

    /// Generic switch between possible UI states. <br>
    /// Valid inputs: "map"
    pub fn generate_ui(state: &String) -> String {

        double_line();

        match state.as_str() {
            "map" => {
                generate_map_cmds();
            },
            _ => println!("Unknown game state {state}"),
        }

        double_line();

        await_user_input()

    }

    fn generate_map_cmds () {
        println!("Use [W][A][S][D] to move.");
    }

    fn await_user_input () -> String {
        println!("Awaiting input: ");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        String::from(user_input)
    }

    fn double_line(){
        println!("==============================================");
    }

    // ==================================================================
    // Input interpretation
    // ==================================================================

    /// Try to convert a user input String into a UserCommand which can be interpreted by the gamestate
    pub fn interpret_input(user_input: String) -> Option<UserCommand> {
        match user_input.as_str() {
            "W" => Some(UserCommand::MOVE(MoveCommand::UP)),
            "A" => Some(UserCommand::MOVE(MoveCommand::LEFT)),
            "S" => Some(UserCommand::MOVE(MoveCommand::DOWN)),
            "D" => Some(UserCommand::MOVE(MoveCommand::RIGHT)),
            "Q" | "Quit" => Some(UserCommand::QUIT),
            _ => None,
        }
    }

}