use std::io;

pub struct Bank {
    pub list: Vec<BankAccount>,
}

pub struct BankAccount {
    owner: String,

    balance: i32,
}

impl Bank {
    // Add a new client

    pub fn add_client(&mut self) {
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

    pub fn display_all(&self) {
        println!("\n--- Account List ---");

        for account in &self.list {
            println!("Owner: {}, Balance: {}$", account.owner, account.balance);
        }
    }

    // Deposit money

    pub fn deposit_money(&mut self) {
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

                match amount_str.trim().parse::<i32>() {
                    Ok(amount) if amount > 0 => {
                        client.balance = client.balance.saturating_add(amount);

                        println!("Deposit successful!");
                    }

                    Ok(_) => println!("Amount must be a positive number."),

                    Err(_) => println!("Invalid amount entered."),
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

pub fn read_io(target: &mut String) {
    target.clear();

    io::stdin().read_line(target).expect("Error reading line");
}
