
mod account;
mod transaction;

use account::{deposit, print_account, CkbAccount};

use transaction::{
    calculate_committed_amount,
    find_transaction,
    get_committed_transactions,
    print_transaction,
    CkbTransaction,
    TransactionStatus,
};

fn main() {
    // 1. Create transactions
    let mut transactions: Vec<CkbTransaction> = Vec::new();

    transactions.push(CkbTransaction {
        tx_hash: String::from("0x111"),
        amount: 1000,
        status: TransactionStatus::Pending,
    });

    transactions.push(CkbTransaction {
        tx_hash: String::from("0x222"),
        amount: 2000,
        status: TransactionStatus::Committed,
    });

    transactions.push(CkbTransaction {
        tx_hash: String::from("0x333"),
        amount: 3000,
        status: TransactionStatus::Rejected,
    });

    transactions.push(CkbTransaction {
        tx_hash: String::from("0x444"),
        amount: 4000,
        status: TransactionStatus::Committed,
    });

    // 2. Display all transactions
    println!("=== All CKB Transactions ===");

    for transaction in &transactions {
        print_transaction(transaction);
    }

    // 3. Search transaction
    println!("\n=== Transaction Search ===");

    match find_transaction(&transactions, "0x222") {
        Some(transaction) => {
            println!("Transaction found!");
            print_transaction(transaction);
        }
        None => println!("Transaction not found!"),
    }

    // 4. Filter committed transactions
    println!("\n=== Committed Transactions ===");

    let committed = get_committed_transactions(&transactions);

    for transaction in &committed {
        println!(
            "{}: {} CKB",
            transaction.tx_hash,
            transaction.amount
        );
    }

    // 5. Calculate statistics
    let total_committed =
        calculate_committed_amount(&transactions);

    println!("\n=== Transaction Statistics ===");
    println!("Total transactions: {}", transactions.len());
    println!("Committed transactions: {}", committed.len());
    println!("Total committed amount: {} CKB", total_committed);

    // 6. Account operations
    let mut account = CkbAccount {
        address: String::from("ckt1-example"),
        balance: 1000,
    };

    println!("\n=== Account Before Deposit ===");
    print_account(&account);

    match deposit(&mut account, 500) {
        Ok(()) => println!("\nDeposit successful!"),
        Err(error) => println!("\nDeposit failed: {}", error),
    }

    println!("\n=== Account After Deposit ===");
    print_account(&account);
}
