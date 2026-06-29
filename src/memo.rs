//! # Store Memos
//! 
//! This module handles storage of memos for import into YNAB.

use std::fmt;

/// Stores a memo.
pub struct Memo{
    memo: String,
}

impl Memo {
    /// Returns an `Option<Memo>`, returning `None` if an empty string is passed.
    /// 
    /// ```
    /// use ynab_import::memo::Memo;
    /// 
    /// let example_memo = Memo::new("Groceries".to_string()).unwrap();
    /// 
    /// assert_eq!(format!("{}", example_memo), "Groceries");
    /// ```
    pub fn new(memo: String) -> Option<Self> {
        match memo.as_str() {
            "" => None,
            _ => Some(Self { memo }),
        }
    }
}

/// Outputs a memo.
impl fmt::Display for Memo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.memo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memo_is_correctly_created() -> Result<(), String> {
        assert!(Memo::new("Test memo.".to_string()).is_some());
        Ok(())
    }

    #[test]
    fn empty_memo_is_not_created() -> Result<(), String> {
        assert!(Memo::new("".to_string()).is_none());
        Ok(())
    }

    #[test]
    fn memo_prints_correctly() -> Result<(), String> {
        let test_memo = Memo::new("Test memo.".to_string()).unwrap();
        assert_eq!(format!("{}", test_memo), "Test memo.".to_string());
        Ok(())
    }
}