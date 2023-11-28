fn main() {
    let mut first_account = BankAccount{
        account_number: 1,
        holder_name: "Sample Person Name 1".to_string(),
        balance: 45
    };

    let mut second_account = BankAccount {
        account_number: 2,
        holder_name: "Sample Person Name 2".to_string(),
        balance: 63
    };

    first_account.deposit(75);
    //println!("{}", first_account.balance);
    second_account.withdraw(18);
    //println!("{}", second_account.balance);

    let first_accounts_balance = first_account.balance();
    let second_accounts_balance = second_account.balance();

    println!("Balance of first_account is {} and balance of second_account {}", first_accounts_balance, second_accounts_balance);
}

trait Account {
    fn deposit(&mut self, deposit: u32);
    fn withdraw(&mut self, withdraw: u32);
    fn balance(&mut self) -> u32;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: u32
}

impl Account for BankAccount{
    fn deposit(&mut self, deposit: u32)
    {
        self.balance = self.balance + deposit;
    }

    fn withdraw(&mut self, withdraw: u32)
    {
        self.balance = self.balance - withdraw;
    }

    fn balance(&mut self) -> u32
    {
        return self.balance;
    }

}