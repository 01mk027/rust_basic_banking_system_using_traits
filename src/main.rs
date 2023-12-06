fn main() {
    //Definition of test account 1
    let mut first_account = BankAccount{
        account_number: 1,
        holder_name: "Sample Person Name 1".to_string(),
        balance: 45
    };

    //Definiton of test account 2
    let mut second_account = BankAccount {
        account_number: 2,
        holder_name: "Sample Person Name 2".to_string(),
        balance: 63
    };

    //Evaluate result of transactions, according to condition, some transactions are performed for test case. match may be useful keyword to deal with quantum computing. 
    match first_account.deposit(-1) {
        Err(err) => println!("An error is occured: {}", err),
        Ok(true) => println!("Transaction is successful and new amount of balance is {}", first_account.balance()),
        Ok(false) => println!("Any other possibilities")
    }

    match second_account.withdraw(83) {
        Err(err) => println!("An error is occured: {}", err),
        Ok(true) => println!("Transaction is successful and new amount of balance is {}", second_account.balance()),
        _ => println!("Any other circumstances")
    }

    //first_account.deposit(75);
    //println!("{}", first_account.balance);
    //second_account.withdraw(18);
    //println!("{}", second_account.balance);

    //let first_accounts_balance = first_account.balance();
    //let second_accounts_balance = second_account.balance();

    //println!("Balance of first_account is {} and balance of second_account {}", first_accounts_balance, second_accounts_balance);
}

//Modified, self methods return Result<T, E> since that
trait Account {
    // If transaction is successul, returns true else Err(message) with message
    fn deposit(&mut self, deposit: i32) -> Result<bool, String>;
    //Same as above
    fn withdraw(&mut self, withdraw: i32) -> Result<bool, String>;
    //Returns amount of balance
    fn balance(&mut self) -> i32;
}

//type of member which is named as balance is set to i32 to allow negative numbers as amount
struct BankAccount {
    account_number: i32,
    holder_name: String,
    balance: i32
}

impl Account for BankAccount{
    fn deposit(&mut self, deposit: i32) -> Result<bool, String>
    {
        //Negative amount can't be considered as deposit number, preventing this...
        if deposit < 0 {
            return Err("Decreasing amount in deposit transaction is impossible".to_string());
        }
        //If amount is positive, routine process is performed and returned true
        self.balance = self.balance + deposit;
        return Ok(true);
    }

    fn withdraw(&mut self, withdraw: i32) -> Result<bool, String>
    {
        //If amount of requested withdraw is greater than balance, this is obstacle for banking system and must be prevented
        if self.balance < withdraw {
            return Err("Amount of requested withdraw reaches amount of balance".to_string());
        }
        //If amount is less than balance, routine process is performed and returned true
        self.balance = self.balance - withdraw;
        return Ok(true);
    }

    //Ordinary function but type of returned to i32 due to traditional results ? :)))
    fn balance(&mut self) -> i32
    {
        return self.balance;
    }

}