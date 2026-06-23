use crate::date::Date;
use crate::payee::Payee;
use crate::memo::Memo;
use crate::flow::Flow;

use std::fmt;

/// Header row of CSV files for import into YNAB.
const HEADER_ROW: &str = "\"Date\",\"Payee\",\"Memo\",\"Outflow\",\"Inflow\"";

/// Stores data about a transacton for import into YNAB.
pub struct ContentRow {
    date: Date,
    payee: Option<Payee>,
    memo: Option<Memo>,
    flow: Flow,
}

impl ContentRow {
    /// Returns a ContentRow, with data about a transaction for import into YNAB.
    pub fn new(date: Date, payee: Option<Payee>, memo: Option<Memo>, flow: Flow) -> Self {
        Self { date, payee, memo, flow }
    }
}

impl fmt::Display for ContentRow {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_of_content_row_with_somes_works() -> Result<(), String> {
        let test_date = Date::new(1995, 3, 8).unwrap();
        let test_payee = Payee::new("The Store".to_string()).unwrap();
        let test_memo = Memo::new("Groceries".to_string()).unwrap();
        let test_flow = Flow::from_amount(-100f64);
        let test_content_row = ContentRow::new(test_date, Some(test_payee), Some(test_memo), test_flow);

        assert_eq!(format!("{}", test_content_row), "\"1995-03-08\",\"The Store\",\"Groceries\",\"100.00\",\"\"");

        Ok(())
    }

    #[test]
    fn creation_of_content_row_with_nones_works() -> Result<(), String> {
        let test_date = Date::new(1995, 3, 8).unwrap();
        let test_flow = Flow::from_amount(100f64);
        let test_content_row = ContentRow::new(test_date, None, None, test_flow);

        assert_eq!(format!("{}", test_content_row), "\"1995-03-08\",\"\",\"\",\"\",\"100.00\"");

        Ok(())
    }
}