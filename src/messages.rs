//! # Error Messages
//! 
//! This module stores error messages in one place, for easy access to make changes to the interface or internationalization easier to perform.

/// Incorrect regex pattern.
pub const UNABLE_TO_PARSE_REGEX: &str = "Unable to parse regular expression. This is not a user fixable error. It needs to be fixed in the source code. Please create an issue on the YNAB Import GitHub repository at https://github.com/JonasEngstrom/ynab-import/issues and describe what you were doing when this problem arose. Sorry for any inconvenience.";

/// Unable to read pipe.
pub const UNABLE_TO_READ_PIPE: &str = "Unable to read input data. Please check that data is correctly piped into program.";

/// Invalid choice of bank.
pub const INVALID_PARSER: &str = "Invalid parser choice. Call program with --help option to see a list of supported parsers.";

/// No input data provided.
pub const NO_INPUT_DATA: &str = "No input data provided. Please provide input data either as a pipe into the program or as a positional argument. Call program with --help for more information.";

/// Input data cannot be parsed.
pub const UNABLE_TO_PARSE_INPUT_DATA: &str = "Unable to parse input data. Please check that the correct bank has been selected, and that the input data has been correctly formatted. Call program with --help for more information.";