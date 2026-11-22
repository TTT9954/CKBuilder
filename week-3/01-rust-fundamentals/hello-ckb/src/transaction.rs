
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Committed,
    Rejected,
}

pub struct CkbTransaction {
    pub tx_hash: String,
    pub amount: u64,
    pub status: TransactionStatus,
}

// Calculate simplified transaction fee
pub fn calculate_fee(amount: u64) -> u64 {
    amount / 1000
}

// Print transaction information
pub fn print_transaction(transaction: &CkbTransaction) {
    println!("\nTransaction: {}", transaction.tx_hash);
    println!("Amount: {} CKB", transaction.amount);
    println!("Estimated Fee: {} CKB", calculate_fee(transaction.amount));

    match transaction.status {
        TransactionStatus::Pending => println!("Status: Pending"),
        TransactionStatus::Committed => println!("Status: Committed"),
        TransactionStatus::Rejected => println!("Status: Rejected"),
    }
}

// Find transaction by hash
pub fn find_transaction<'a>(
    transactions: &'a [CkbTransaction],
    tx_hash: &str,
) -> Option<&'a CkbTransaction> {
    transactions.iter().find(|tx| tx.tx_hash == tx_hash)
}

// Filter committed transactions
pub fn get_committed_transactions(
    transactions: &[CkbTransaction],
) -> Vec<&CkbTransaction> {
    transactions
        .iter()
        .filter(|tx| tx.status == TransactionStatus::Committed)
        .collect()
}

// Calculate total committed amount
pub fn calculate_committed_amount(
    transactions: &[CkbTransaction],
) -> u64 {
    transactions
        .iter()
        .filter(|tx| tx.status == TransactionStatus::Committed)
        .map(|tx| tx.amount)
        .sum()
}