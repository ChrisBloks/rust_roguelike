pub mod user_interface {

    use std::io;


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


}