//! # Store Payees
//! 
//! This module handles storage of payees for import into YNAB.

use std::fmt;

/// Stores a payee.
pub struct Payee{
    payee: String,
}

impl Payee {
    /// Returns an `Option<Payee>`, returning `None` if an empty string is passed.
    /// 
    /// ```
    /// use ynab_import::payee::Payee;
    /// 
    /// let example_payee = Payee::new("The Grocery Store".to_string()).unwrap();
    /// 
    /// assert_eq!(format!("{}", example_payee), "The Grocery Store");
    /// ```
    pub fn new(payee: String) -> Option<Self> {
        match payee.as_str() {
            "" => None,
            _ => Some(Self { payee }),
        }
    }
}

/// Outputs a payee.
impl fmt::Display for Payee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.payee)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payee_is_correctly_created() -> Result<(), String> {
        assert!(Payee::new("Test payee.".to_string()).is_some());
        Ok(())
    }

    #[test]
    fn empty_payee_is_not_created() -> Result<(), String> {
        assert!(Payee::new("".to_string()).is_none());
        Ok(())
    }

    #[test]
    fn payee_prints_correctly() -> Result<(), String> {
        let test_payee = Payee::new("Test payee.".to_string()).unwrap();
        assert_eq!(format!("{}", test_payee), "Test payee.");
        Ok(())
    }
}