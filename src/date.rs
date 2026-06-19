use std::error::Error;
use std::fmt;

/// Indicates an attempt to create a year out of the range 0000 to 9999 inclusive.
#[derive(Debug)]
struct YearOutOfRangeError;

impl fmt::Display for YearOutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Year needs to be in range 0000 to 9999.")
    }
}

impl Error for YearOutOfRangeError {}

/// Indicates an attempt to create a month out of the range 1 to 12 inclusive.
#[derive(Debug)]
struct MonthOutOfRangeError;

impl fmt::Display for MonthOutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Month needs to be in range 1 to 12.")
    }
}

impl Error for MonthOutOfRangeError {}

/// Indicates an attempt to create a day out of the range 1 to 31 inclusive or one that has fore days than an associated month, when creating a Date.
#[derive(Debug)]
struct DayOutOfRangeError;

impl fmt::Display for DayOutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Day needs to be in range 1 to 31.")
    }
}

impl Error for DayOutOfRangeError {}

/// Stores a year between 0000 and 9999.
struct Year {
    year: u16,
}

impl Year {
    /// Returns a result. If the year is in the range 0000 to 9999 inclusive, an `Ok(Year)` is returned. Otherwise an `Err(YearOutOfRangeError)` is returned.
    pub fn new(year: u16) -> Result<Self, YearOutOfRangeError>  {
        match year {
            year @ 0 ..= 9999 => Ok(Year { year }),
            _ => Err(YearOutOfRangeError),
        }
    }

    /// Returns true if the year is a leap year. Otherwise returns false.
    pub fn is_leap_year(&self) -> bool {
        match (self.year % 4 == 0, self.year % 100 == 0, self.year % 400 == 0) {
            (true, true, true) => true,
            (true, true, false) => false,
            (true, false, false) => true,
            (false, false, false) => false,
            _ => unreachable!("All possible years should be handled by previous match arms."),
        }
    }
}

/// Outputs Year with leading zeros, as needed.
impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}", self.year)
    }
}

/// Stores a month between 1 and 12.
struct Month {
    month: u8,
}

impl Month {
    /// Returns a result. If the month is in the range 1 to 21 inclusive, an `Ok(Month)` is returned. Otherwise an `Err(MonthOutOfRangeError)` is returned.
    pub fn new(month: u8) -> Result<Self, MonthOutOfRangeError> {
        match month {
            month @ 1 ..= 12 => Ok(Month { month }),
            _ => Err(MonthOutOfRangeError),
        }
    }
}

/// Outputs a Month with a leading zero, as needed.
impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}", self.month)
    }
}

/// Stores a day between 1 and 31.
struct Day {
    day: u8,
}

impl Day {
    /// Returns a result. If the day is in the range 1 to 31, an `Ok(Day)` is returned. Otherwise and `Err(DayOutOfRangeError)` is returned.
    pub fn new(day: u8) -> Result<Self, DayOutOfRangeError> {
        match day {
            day @ 1 ..= 31 => Ok(Day { day }),
            _ => Err(DayOutOfRangeError),
        }
    }
}

/// Outputs a Day with a leading zero, as needed.
impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}", self.day)
    }
}

/// Stores a date between 0000-01-01 and 9999-12-31.
pub struct Date {
    year: Year,
    month: Month,
    day: Day,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Result<Self, Box<dyn Error>> {
        let date = Self { year: Year::new(year)?, month: Month::new(month)?, day: Day::new(day)? };
        match date.month.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => Ok(date),
            4 | 6 | 9 | 11 => {
                match date.day.day {
                    _day @ 1 ..= 30 => Ok(date),
                    _ => Err(Box::new(DayOutOfRangeError)),
                }
            },
            2 => {
                match date.year.is_leap_year() {
                    true => match date.day.day {
                        _day @ 1 ..= 29 => Ok(date),
                        _ => Err(Box::new(DayOutOfRangeError)),
                    },
                    false => match date.day.day {
                        _day @ 1 ..= 28 => Ok(date),
                        _ => Err(Box::new(DayOutOfRangeError)),
                    },
                }
            },
            _ => unreachable!("All possible days should be handled by previous match arms in combination with error handling in the Day type."),
        }
    }
}

/// Outputs a Date in ISO format.
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_in_range_year_works() -> Result<(), YearOutOfRangeError> {
        Year::new(0)?;
        Ok(())
    }

    #[test]
    fn maximum_in_range_year_works() -> Result<(), YearOutOfRangeError> {
        Year::new(9999)?;
        Ok(())
    }

    #[test]
    fn out_of_range_year_errors() -> Result<(), String> {
        let test_year = Year::new(10000);
        assert!(matches!(test_year, Err(YearOutOfRangeError)));
        Ok(())
    }

    #[test]
    fn year_with_leading_zeros_prints_correctly() -> Result<(), YearOutOfRangeError> {
        let test_year = Year::new(1);
        assert_eq!(format!("{}", test_year?), "0001");
        Ok(())
    }

    #[test]
    fn year_without_leading_zeros_prints_correctly() -> Result<(), YearOutOfRangeError> {
        let test_year = Year::new(5555);
        assert_eq!(format!("{}", test_year?), "5555");
        Ok(())
    }

    #[test]
    fn the_year_1600_is_a_leap_year() -> Result<(), YearOutOfRangeError> {
        assert!(Year::new(1600)?.is_leap_year());
        Ok(())
    }

    #[test]
    fn the_year_2000_is_a_leap_year() -> Result<(), YearOutOfRangeError> {
        assert!(Year::new(2000)?.is_leap_year());
        Ok(())
    }

    #[test]
    fn the_year_2400_is_a_leap_year() -> Result<(), YearOutOfRangeError> {
        assert!(Year::new(2400)?.is_leap_year());
        Ok(())
    }

    #[test]
    fn the_year_1700_is_not_a_leap_year() -> Result<(), YearOutOfRangeError> {
        assert!(!Year::new(1700)?.is_leap_year());
        Ok(())
    }

    #[test]
    fn the_year_1800_is_not_a_leap_year() -> Result<(), YearOutOfRangeError> {
        assert!(!Year::new(1800)?.is_leap_year());
        Ok(())
    }

    #[test]
    fn the_year_1900_is_not_a_leap_year() -> Result<(), YearOutOfRangeError> {
        assert!(!Year::new(1900)?.is_leap_year());
        Ok(())
    }

    #[test]
    fn the_year_2100_is_not_a_leap_year() -> Result<(), YearOutOfRangeError> {
        assert!(!Year::new(2100)?.is_leap_year());
        Ok(())
    }

    #[test]
    fn the_year_2200_is_not_a_leap_year() -> Result<(), YearOutOfRangeError> {
        assert!(!Year::new(2200)?.is_leap_year());
        Ok(())
    }

    #[test]
    fn the_year_1993_is_not_a_leap_year() -> Result<(), YearOutOfRangeError> {
        assert!(!Year::new(1993)?.is_leap_year());
        Ok(())
    }

    #[test]
    fn the_year_1996_is_a_leap_year() -> Result<(), YearOutOfRangeError> {
        assert!(Year::new(1996)?.is_leap_year());
        Ok(())
    }

    #[test]
    fn minimum_in_range_month_works() -> Result<(), MonthOutOfRangeError> {
        Month::new(1)?;
        Ok(())
    }

    #[test]
    fn maximum_in_range_month_works() -> Result<(), MonthOutOfRangeError> {
        Month::new(12)?;
        Ok(())
    }

    #[test]
    fn out_of_range_month_errors() -> Result<(), String> {
        let test_month = Month::new(13);
        assert!(matches!(test_month, Err(MonthOutOfRangeError)));
        Ok(())
    }

    #[test]
    fn month_with_leading_zero_prints_correctly() -> Result<(), MonthOutOfRangeError> {
        let test_month = Month::new(1);
        assert_eq!(format!("{}", test_month?), "01");
        Ok(())
    }

    #[test]
    fn month_without_leading_zero_prints_correctly() -> Result<(), MonthOutOfRangeError> {
        let test_month = Month::new(12);
        assert_eq!(format!("{}", test_month?), "12");
        Ok(())
    }

    #[test]
    fn minimum_in_range_day_works() -> Result<(), DayOutOfRangeError> {
        Day::new(1)?;
        Ok(())
    }

    #[test]
    fn maximum_in_range_day_works() -> Result<(), DayOutOfRangeError> {
        Day::new(31)?;
        Ok(())
    }

    #[test]
    fn out_of_range_day_errors() -> Result<(), String> {
        let test_day = Day::new(32);
        assert!(matches!(test_day, Err(DayOutOfRangeError)));
        Ok(())
    }

    #[test]
    fn day_with_leading_zero_prints_correctly() -> Result<(), DayOutOfRangeError> {
        let test_day = Day::new(1);
        assert_eq!(format!("{}", test_day?), "01");
        Ok(())
    }

    #[test]
    fn day_without_leading_zero_prints_correctly() -> Result<(), DayOutOfRangeError> {
        let test_day = Day::new(15);
        assert_eq!(format!("{}", test_day?), "15");
        Ok(())
    }

    #[test]
    fn date_prints_in_correct_iso_format_without_leading_zeros() -> Result<(), Box<dyn Error>> {
        assert_eq!(format!("{}", Date::new(1995, 12, 30)?), "1995-12-30");
        Ok(())
    }

    #[test]
    fn date_prints_in_correct_iso_format_with_leading_zeros() -> Result<(), Box<dyn Error>> {
        assert_eq!(format!("{}", Date::new(995, 8, 3)?), "0995-08-03");
        Ok(())
    }

    #[test]
    fn january_date_can_be_created_correctly() -> Result<(), Box<dyn Error>> {
        Date::new(1995, 01, 31)?;
        Ok(())
    }

    #[test]
    fn february_non_leap_year_date_can_be_created_correctly() -> Result<(), Box<dyn Error>> {
        Date::new(1995, 02, 28)?;
        Ok(())
    }

    #[test]
    fn february_leap_year_date_can_be_created_correctly() -> Result<(), Box<dyn Error>> {
        Date::new(1996, 02, 29)?;
        Ok(())
    }

    #[test]
    fn march_date_can_be_created_correctly() -> Result<(), Box<dyn Error>> {
        Date::new(1995, 03, 31)?;
        Ok(())
    }

    #[test]
    fn april_date_can_be_created_correctly() -> Result<(), Box<dyn Error>> {
        Date::new(1995, 04, 30)?;
        Ok(())
    }

    #[test]
    fn may_date_can_be_created_correctly() -> Result<(), Box<dyn Error>> {
        Date::new(1995, 05, 31)?;
        Ok(())
    }

    #[test]
    fn june_date_can_be_created_correctly() -> Result<(), Box<dyn Error>> {
        Date::new(1995, 06, 30)?;
        Ok(())
    }

    #[test]
    fn july_date_can_be_created_correctly() -> Result<(), Box<dyn Error>> {
        Date::new(1995, 07, 31)?;
        Ok(())
    }

    #[test]
    fn august_date_can_be_created_correctly() -> Result<(), Box<dyn Error>> {
        Date::new(1995, 08, 31)?;
        Ok(())
    }

    #[test]
    fn september_date_can_be_created_correctly() -> Result<(), Box<dyn Error>> {
        Date::new(1995, 09, 30)?;
        Ok(())
    }

    #[test]
    fn october_date_can_be_created_correctly() -> Result<(), Box<dyn Error>> {
        Date::new(1995, 10, 31)?;
        Ok(())
    }

    #[test]
    fn november_date_can_be_created_correctly() -> Result<(), Box<dyn Error>> {
        Date::new(1995, 11, 30)?;
        Ok(())
    }

    #[test]
    fn december_date_can_be_created_correctly() -> Result<(), Box<dyn Error>> {
        Date::new(1995, 12, 31)?;
        Ok(())
    }

    #[test]
    fn january_date_errors_correctly() -> Result<(), Box<dyn Error>> {
        let test_date = Date::new(1995, 01, 32);
        assert!(test_date.err().is_some_and(|e| e.is::<DayOutOfRangeError>()));
        Ok(())
    }

    #[test]
    fn leap_year_february_date_errors_correctly() -> Result<(), Box<dyn Error>> {
        let test_date = Date::new(1996, 2, 30);
        assert!(test_date.err().is_some_and(|e| e.is::<DayOutOfRangeError>()));
        Ok(())
    }

    #[test]
    fn non_leap_year_february_date_errors_correctly() -> Result<(), Box<dyn Error>> {
        let test_date = Date::new(1995, 2, 29);
        assert!(test_date.err().is_some_and(|e| e.is::<DayOutOfRangeError>()));
        Ok(())
    }

    #[test]
    fn march_date_errors_correctly() -> Result<(), Box<dyn Error>> {
        let test_date = Date::new(1995, 3, 32);
        assert!(test_date.err().is_some_and(|e| e.is::<DayOutOfRangeError>()));
        Ok(())
    }

    #[test]
    fn april_date_errors_correctly() -> Result<(), Box<dyn Error>> {
        let test_date = Date::new(1995, 4, 31);
        assert!(test_date.err().is_some_and(|e| e.is::<DayOutOfRangeError>()));
        Ok(())
    }

    #[test]
    fn may_date_errors_correctly() -> Result<(), Box<dyn Error>> {
        let test_date = Date::new(1995, 5, 32);
        assert!(test_date.err().is_some_and(|e| e.is::<DayOutOfRangeError>()));
        Ok(())
    }

    #[test]
    fn june_date_errors_correctly() -> Result<(), Box<dyn Error>> {
        let test_date = Date::new(1995, 6, 31);
        assert!(test_date.err().is_some_and(|e| e.is::<DayOutOfRangeError>()));
        Ok(())
    }

    #[test]
    fn july_date_errors_correctly() -> Result<(), Box<dyn Error>> {
        let test_date = Date::new(1995, 7, 32);
        assert!(test_date.err().is_some_and(|e| e.is::<DayOutOfRangeError>()));
        Ok(())
    }

    #[test]
    fn august_date_errors_correctly() -> Result<(), Box<dyn Error>> {
        let test_date = Date::new(1995, 8, 32);
        assert!(test_date.err().is_some_and(|e| e.is::<DayOutOfRangeError>()));
        Ok(())
    }

    #[test]
    fn september_date_errors_correctly() -> Result<(), Box<dyn Error>> {
        let test_date = Date::new(1995, 9, 31);
        assert!(test_date.err().is_some_and(|e| e.is::<DayOutOfRangeError>()));
        Ok(())
    }

    #[test]
    fn october_date_errors_correctly() -> Result<(), Box<dyn Error>> {
        let test_date = Date::new(1995, 10, 32);
        assert!(test_date.err().is_some_and(|e| e.is::<DayOutOfRangeError>()));
        Ok(())
    }

    #[test]
    fn november_date_errors_correctly() -> Result<(), Box<dyn Error>> {
        let test_date = Date::new(1995, 11, 31);
        assert!(test_date.err().is_some_and(|e| e.is::<DayOutOfRangeError>()));
        Ok(())
    }

    #[test]
    fn december_date_errors_correctly() -> Result<(), Box<dyn Error>> {
        let test_date = Date::new(1995, 12, 32);
        assert!(test_date.err().is_some_and(|e| e.is::<DayOutOfRangeError>()));
        Ok(())
    }
}