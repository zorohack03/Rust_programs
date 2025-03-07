struct BankAccount {
    account_number: String,
    owner_name: String,
    balance: f64,
}

impl BankAccount {
    fn new(account_number: &str, owner_name: &str, balance: f64) -> Self {
        BankAccount {
            account_number: account_number.to_string(),
            owner_name: owner_name.to_string(),
            balance,
        }
    }

    fn view_balance(&self) -> f64 {
        self.balance
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if self.balance >= amount {
            self.balance -= amount;
        } else {
            println!("Insufficient funds!");
        }
    }
}

fn main() {
    let mut account = BankAccount::new("12345", "Muthra", 1000.0);

    println!("Initial balance: {}", account.view_balance());
    
    account.deposit(500.0);
    println!("After deposit: {}", account.view_balance());

    account.withdraw(200.0);
    println!("After withdrawal: {}", account.view_balance());
}

