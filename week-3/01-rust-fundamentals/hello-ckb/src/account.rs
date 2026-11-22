
pub struct CkbAccount {
    pub address: String,
    pub balance: u64,
}

// Validate transaction amount
pub fn validate_amount(amount: u64) -> Result<(), String> {
    if amount == 0 {
        return Err(String::from(
            "Amount must be greater than zero"
        ));
    }

    Ok(())
}

// Deposit with overflow protection
pub fn deposit(
    account: &mut CkbAccount,
    amount: u64,
) -> Result<(), String> {
    validate_amount(amount)?;

    account.balance = account.balance
        .checked_add(amount)
        .ok_or(String::from("Balance overflow"))?;

    Ok(())
}

// Print account information
pub fn print_account(account: &CkbAccount) {
    println!("Address: {}", account.address);
    println!("Balance: {} CKB", account.balance);
}
