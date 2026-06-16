use std::io;

struct Bank {
    list: Vec<BankAccount>,
}

struct BankAccount {
    owner: String,
    balance: i32,
}

impl Bank {
    // Add a new client
    fn add_client(&mut self) {
        let mut name = String::new();
        println!("Enter the client's name:");
        read_io(&mut name);

        let new_account = BankAccount {
            owner: name.trim().to_string(),
            balance: 0,
        };

        self.list.push(new_account);
        println!("Client added successfully.");
    }

    // Display all clients
    fn display_all(&self) {
        println!("\n--- Account List ---");
        for account in &self.list {
            println!("Owner: {}, Balance: {}$", account.owner, account.balance);
        }
    }

    // Deposit money
    fn deposit_money(&mut self) {
        println!("Enter the client's name for the deposit:");
        let mut name = String::new();
        read_io(&mut name);
        let name = name.trim();

        let mut found = false;
        for client in &mut self.list {
            if client.owner == name {
                println!("Enter amount:");
                let mut amount_str = String::new();
                read_io(&mut amount_str);

                if let Ok(amount) = amount_str.trim().parse::<i32>() {
                    client.balance += amount;
                    println!("Deposit successful!");
                }
                found = true;
                break; 
            }
        }

        if !found {
            println!("Client not found.");
        }
    }
}

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

fn read_io(target: &mut String) {
    target.clear();
    io::stdin().read_line(target).expect("Error reading line");
}
