pub enum MenuActions {
    AddClient,
    Deposit,
    Display,
    Exit,
    Invalid,
}

impl MenuActions {
    pub fn from_input(input: &str) -> Self {
        match input.trim() {
            "1" => MenuActions::AddClient,
            "2" => MenuActions::Deposit,
            "3" => MenuActions::Display,
            "4" => MenuActions::Exit,
            _   => MenuActions::Invalid, 
        }
    }
}