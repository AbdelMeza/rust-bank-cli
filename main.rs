mod bank;
use bank::Bank;
use bank::read_io;

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

        match choice.trim() {
            "1" => bank.add_client(),
            "2" => bank.deposit_money(),
            "3" => bank.display_all(),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}