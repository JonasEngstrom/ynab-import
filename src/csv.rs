use crate::date::Date;
use crate::payee::Payee;
use crate::memo::Memo;
use crate::flow::Flow;

use std::fmt;

/// Header row of CSV files for import into YNAB.
const HEADER_ROW: [&str; 5] = [
    "Date",
    "Payee",
    "Memo",
    "Outflow",
    "Inflow"
];

/// Stores data about a transaction for import into YNAB.
pub struct Transaction {
    date: Date,
    payee: Option<Payee>,
    memo: Option<Memo>,
    flow: Flow,
}

impl Transaction {
    /// Returns a Transaction, with data necessary for creation of [a CSV row for import into YNAB](https://support.ynab.com/en_us/formatting-a-csv-file-an-overview-BJvczkuRq).
    pub fn new(date: Date, payee: Option<Payee>, memo: Option<Memo>, flow: Flow) -> Self {
        Self { date, payee, memo, flow }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "\"{}\",\"{}\",\"{}\",\"{}\"",
            self.date,
            match &self.payee {
                Some(payee) => format!("{}", payee),
                _ => "".to_string(),
            },
            match &self.memo {
                Some(memo) => format!("{}", memo),
                _ => "".to_string(),
            },
            self.flow
        )
    }
}

/// Stores a list of Transactions in preparation for formatting as CSV for import into YNAB.
pub struct TransactionList {
    transaction_list: Vec<Transaction>,
}

impl TransactionList {
    /// Returns an empty TransactionList.
    pub fn new() -> Self {
        Self { transaction_list: Vec::<Transaction>::new() }
    }

    /// Adds a Transaction to a TransactionList.
    pub fn push(&mut self, transaction: Transaction) -> () {
        self.transaction_list.push(transaction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_of_content_row_with_somes_works() -> Result<(), String> {
        let test_date = Date::new(1995, 3, 8).unwrap();
        let test_payee = Payee::new("The Store".to_string()).unwrap();
        let test_memo = Memo::new("Groceries".to_string()).unwrap();
        let test_flow = Flow::from_amount(-100f64);
        let test_transaction = Transaction::new(test_date, Some(test_payee), Some(test_memo), test_flow);

        assert_eq!(format!("{}", test_transaction), "\"1995-03-08\",\"The Store\",\"Groceries\",\"100.00\",\"\"");

        Ok(())
    }

    #[test]
    fn creation_of_content_row_with_nones_works() -> Result<(), String> {
        let test_date = Date::new(1995, 3, 8).unwrap();
        let test_flow = Flow::from_amount(100f64);
        let test_transaction = Transaction::new(test_date, None, None, test_flow);

        assert_eq!(format!("{}", test_transaction), "\"1995-03-08\",\"\",\"\",\"\",\"100.00\"");

        Ok(())
    }

    #[test]
    fn creation_of_transaction_list_works() -> Result<(), String> {
        let test_transaction_list = TransactionList::new();

        assert!(test_transaction_list.transaction_list.is_empty());

        Ok(())
    }

        #[test]
    fn pushing_to_transaction_list_works() -> Result<(), String> {
        let test_date = Date::new(1995, 3, 8).unwrap();
        let test_payee = Payee::new("The Store".to_string()).unwrap();
        let test_memo = Memo::new("Groceries".to_string()).unwrap();
        let test_flow = Flow::from_amount(-100f64);
        let test_transaction = Transaction::new(test_date, Some(test_payee), Some(test_memo), test_flow);
        
        let mut test_transaction_list = TransactionList::new();

        test_transaction_list.push(test_transaction);

        assert_eq!(format!("{}", test_transaction_list.transaction_list.first().unwrap()), "\"1995-03-08\",\"The Store\",\"Groceries\",\"100.00\",\"\"");

        Ok(())
    }
}