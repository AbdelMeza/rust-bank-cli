mod bank;
mod menu;
use bank::Bank;
use bank::read_io;
use menu::MenuActions;

fn main() {
    let mut bank = Bank { list: Vec::new() };

    loop {
        println!("\n--- BANKING MENU ---");
        println!("1. Add a client");
        println!("2. Deposit money");
        println!("3. Display accounts");
        println!("4. Exit");
        print!("Your choice: ");

        let mut choice = String::new();
        read_io(&mut choice);

        match MenuActions::from_input(&choice) {
            MenuActions::AddClient => bank.add_client(),
            MenuActions::Deposit => bank.deposit_money(),
            MenuActions::Display => bank.display_all(),
            MenuActions::Exit => {
                println!("Goodbye!");
                break;
            }
            MenuActions::Invalid => println!("Invalid choice, please try again."),
        }
    }
}