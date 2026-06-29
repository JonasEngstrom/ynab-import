//! # Store Inflows and Outflows
//! 
//! This module handles creating and storing inflow and outflow pairs.

use std::fmt;

/// Stores an outflow/inflow pair.
pub struct Flow {
    outflow: Option<f64>,
    inflow: Option<f64>,
}

impl Flow {
    /// Returns a Flow, based on a positive or negative amount, yielding an outflow if the amount is negative, and an inflow if the amount is positive. The fields are stored as `Option<f64>`, returning `None` if the other flow is positive. If the input amount is zero, both fields are `None`.
    /// 
    /// ```
    /// use ynab_import::flow::Flow;
    /// 
    /// let zero_flow = Flow::from_amount(0f64);
    /// 
    /// assert_eq!(format!("{}", zero_flow), "\",\"");
    /// ```
    /// 
    /// ```
    /// use ynab_import::flow::Flow;
    /// 
    /// let outflow = Flow::from_amount(-15f64);
    /// 
    /// assert_eq!(format!("{}", outflow), "15.00\",\"");
    /// ```
    /// 
    /// ```
    /// use ynab_import::flow::Flow;
    /// 
    /// let inflow = Flow::from_amount(15f64);
    /// 
    /// assert_eq!(format!("{}", inflow), "\",\"15.00");
    /// ```
    pub fn from_amount(amount: f64) -> Self {
        match amount {
            0f64 => Self { outflow: None, inflow: None },
            amount @ ..0f64 => Self { outflow: Some(amount.abs()), inflow: None },
            amount @ 0f64.. => Self { outflow: None, inflow: Some(amount) },
            _ => unreachable!("All possible values should be handled by previous match arms."),
        }
    }

    /// Returns a Flow, based on a positive or negative outflow, yielding an inflow if the amount is negative, and an outflow if the amount is positive. The fields are stored as `Option<f64>`, returning `None` if the other flow is positive. If the input amount is zero, both fields are `None`.
    /// 
    /// ```
    /// use ynab_import::flow::Flow;
    /// 
    /// let zero_flow = Flow::from_outflow(0f64);
    /// 
    /// assert_eq!(format!("{}", zero_flow), "\",\"");
    /// ```
    /// 
    /// ```
    /// use ynab_import::flow::Flow;
    /// 
    /// let inflow = Flow::from_outflow(-15f64);
    /// 
    /// assert_eq!(format!("{}", inflow), "\",\"15.00");
    /// ```
    /// 
    /// ```
    /// use ynab_import::flow::Flow;
    /// 
    /// let outflow = Flow::from_outflow(15f64);
    /// 
    /// assert_eq!(format!("{}", outflow), "15.00\",\"");
    /// ```
    pub fn from_outflow(outflow: f64) -> Self {
        Self::from_amount(-outflow)
    }

    /// Returns a Flow, based on a positive or negative inflow, yielding an outflow if the amount is negative, and an inflow if the amount is positive. The fields are stored as `Option<f64>`, returning `None` if the other flow is positive. If the input amount is zero, both fields are `None`. Convenience function, calling `Flow::from_amount()`.
    /// 
    /// ```
    /// use ynab_import::flow::Flow;
    /// 
    /// let zero_flow = Flow::from_inflow(0f64);
    /// 
    /// assert_eq!(format!("{}", zero_flow), "\",\"");
    /// ```
    /// 
    /// ```
    /// use ynab_import::flow::Flow;
    /// 
    /// let outflow = Flow::from_inflow(-15f64);
    /// 
    /// assert_eq!(format!("{}", outflow), "15.00\",\"");
    /// ```
    /// 
    /// ```
    /// use ynab_import::flow::Flow;
    /// 
    /// let inflow = Flow::from_inflow(15f64);
    /// 
    /// assert_eq!(format!("{}", inflow), "\",\"15.00");
    /// ```
    pub fn from_inflow(inflow: f64) -> Self {
        Self::from_amount(inflow)
    }
}

/// Outputs an outflow/inflow pair.
impl fmt::Display for Flow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let outflow = match self.outflow {
            Some(outflow) => format!("{:.2}", outflow),
            None => "".to_string(),
        };
        let inflow = match self.inflow {
            Some(inflow) => format!("{:.2}", inflow),
            None => "".to_string(),
        };
        write!(f, "{}\",\"{}", outflow, inflow)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_amount_handled_correctly() -> Result<(), String> {
        let test_flow = Flow::from_amount(0f64);

        assert_eq!(test_flow.outflow, None);
        assert_eq!(test_flow.inflow, None);
        
        Ok(())
    }

    #[test]
    fn negative_amount_handled_correctly() -> Result<(), String> {
        let test_flow = Flow::from_amount(-15f64);

        assert_eq!(test_flow.outflow.unwrap(), 15f64);
        assert_eq!(test_flow.inflow, None);
        
        Ok(())
    }

    #[test]
    fn positive_amount_handled_correctly() -> Result<(), String> {
        let test_flow = Flow::from_amount(15f64);

        assert_eq!(test_flow.outflow, None);
        assert_eq!(test_flow.inflow.unwrap(), 15f64);
        
        Ok(())
    }

    #[test]
    fn zero_outflow_handled_correctly() -> Result<(), String> {
        let test_flow = Flow::from_outflow(0f64);

        assert_eq!(test_flow.outflow, None);
        assert_eq!(test_flow.inflow, None);
        
        Ok(())
    }

    #[test]
    fn negative_outflow_handled_correctly() -> Result<(), String> {
        let test_flow = Flow::from_outflow(-15f64);

        assert_eq!(test_flow.outflow, None);
        assert_eq!(test_flow.inflow.unwrap(), 15f64);
        
        Ok(())
    }

    #[test]
    fn positive_outflow_handled_correctly() -> Result<(), String> {
        let test_flow = Flow::from_outflow(15f64);

        assert_eq!(test_flow.outflow.unwrap(), 15f64);
        assert_eq!(test_flow.inflow, None);
        
        Ok(())
    }

    #[test]
    fn zero_inflow_handled_correctly() -> Result<(), String> {
        let test_flow = Flow::from_inflow(0f64);

        assert_eq!(test_flow.outflow, None);
        assert_eq!(test_flow.inflow, None);
        
        Ok(())
    }

    #[test]
    fn negative_inflow_handled_correctly() -> Result<(), String> {
        let test_flow = Flow::from_inflow(-15f64);

        assert_eq!(test_flow.outflow.unwrap(), 15f64);
        assert_eq!(test_flow.inflow, None);
        
        Ok(())
    }

    #[test]
    fn positive_inflow_handled_correctly() -> Result<(), String> {
        let test_flow = Flow::from_inflow(15f64);

        assert_eq!(test_flow.outflow, None);
        assert_eq!(test_flow.inflow.unwrap(), 15f64);
        
        Ok(())
    }

    #[test]
    fn zero_flow_formats_correctly() -> Result<(), String> {
        let test_flow = Flow::from_amount(0f64);

        assert_eq!(format!("{}", test_flow), "\",\"");

        Ok(())
    }

    #[test]
    fn outflow_formats_correctly() -> Result<(), String> {
        let test_flow = Flow::from_amount(-15f64);

        assert_eq!(format!("{}", test_flow), "15.00\",\"");

        Ok(())
    }

    #[test]
    fn inflow_formats_correctly() -> Result<(), String> {
        let test_flow = Flow::from_amount(15f64);

        assert_eq!(format!("{}", test_flow), "\",\"15.00");

        Ok(())
    }
}