use clap::Parser;
use std::io::{ Read, IsTerminal };
use crate::messages;
use std::process::exit;
use crate::parser;

#[derive(Parser)]
#[command(version, about)]
struct Arguments {
    /// Output from the bank to be formatted into CSV format for import into YNAB. If argument is not provided, data is read from stdin instead, to enable piping into the program. Note that this means that if this argument is provided any pipes are ignored.
    text_to_parse: Option<String>,

    /// Name of bank from which data were copied. Supported options are 'danske-bank'/'db' (data copied from Danske Bank A/S, Danmark, Sverige Filial’s Hembanken)
    #[arg(short, long, value_name = "BANK NAME")]
    bank_name: String,

    /// Optional memo to assign to all of the imported transactions.
    #[arg(short, long, value_name = "MEMO")]
    memo: Option<String>,
}

/// Run main program logic.
pub fn run() -> () {
    let arguments = Arguments::parse();

    let parser = match arguments.bank_name.as_str() {
        "danske-bank" | "db" => parser::Parser::DanskeBank,
        _ => {
            eprintln!("{}", messages::INVALID_PARSER);
            exit(1);
        },
    };

    let memo = arguments.memo.as_deref();

    let text_to_parse = match arguments.text_to_parse {
        Some(text) => text,
        None => match read_pipe() {
            Some(text) => text,
            None => {
                eprintln!("{}", messages::NO_INPUT_DATA);
                exit(1);
            },
        },
    };

    let output = match parser.parse(&text_to_parse, memo) {
        Ok(text) => text,
        Err(_) => {
            eprintln!("{}", messages::UNABLE_TO_PARSE_INPUT_DATA);
            exit(1);
        },
    };

    println!("{}", output);
}

fn read_pipe() -> Option<String> {
    let mut buffer = String::new();
    if !std::io::stdin().is_terminal() {
        match std::io::stdin().read_to_string(&mut buffer) {
            Ok(_) => {
                match buffer.trim().is_empty() {
                    true => return None,
                    false => return Some(buffer.trim().to_string()),
                };
            },
            Err(_) => {
                eprintln!("{}", messages::UNABLE_TO_READ_PIPE);
                exit(1);
            },
        }
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::path::PathBuf;
    use std::process::{Command, Stdio};

    fn binary_path() -> PathBuf {
        std::env::var_os("CARGO_BIN_EXE_ynab-import")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/debug/ynab-import"))
    }

    fn binary() -> Command {
        Command::new(binary_path())
    }

    fn danske_bank_input() -> &'static str {
        r#"2026-06-25				
Foo Bar ))))
-41,45
 
 		62.930,69		"#
    }

    #[test]
    fn prints_csv_for_direct_input() {
        let output = binary()
            .args(["2026-06-25\t\t\t\t\nFoo Bar ))))\n-41,45\n \n \t	62.930,69\t\t", "--bank-name", "db", "--memo", "A short memo"])
            .output()
            .expect("binary should run");

        assert!(output.status.success());
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "\"Date\",\"Payee\",\"Memo\",\"Outflow\",\"Inflow\"\n\"2026-06-25\",\"Foo Bar\",\"A short memo\",\"41.45\",\"\"\n");
        assert!(String::from_utf8(output.stderr).unwrap().is_empty());
    }

    #[test]
    fn prints_csv_for_piped_input() {
        let mut child = binary()
            .args(["--bank-name", "danske-bank"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("binary should spawn");

        child
            .stdin
            .as_mut()
            .expect("stdin should be piped")
            .write_all(danske_bank_input().as_bytes())
            .expect("stdin should accept input");

        let output = child.wait_with_output().expect("binary should finish");

        assert!(output.status.success());
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "\"Date\",\"Payee\",\"Memo\",\"Outflow\",\"Inflow\"\n\"2026-06-25\",\"Foo Bar\",\"\",\"41.45\",\"\"\n");
        assert!(String::from_utf8(output.stderr).unwrap().is_empty());
    }

    #[test]
    fn rejects_invalid_parser_name() {
        let output = binary()
            .args(["--bank-name", "unknown"])
            .output()
            .expect("binary should run");

        assert!(!output.status.success());
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
        assert_eq!(String::from_utf8(output.stderr).unwrap().trim_end(), "Invalid parser choice. Call program with --help option to see a list of supported parsers.");
    }

    #[test]
    fn rejects_missing_input_data() {
        let output = binary()
            .args(["--bank-name", "db"])
            .output()
            .expect("binary should run");

        assert!(!output.status.success());
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
        assert_eq!(String::from_utf8(output.stderr).unwrap().trim_end(), "No input data provided. Please provide input data either as a pipe into the program or as a positional argument. Call program with --help for more information.");
    }

    #[test]
    fn rejects_unparseable_input() {
        let output = binary()
            .args(["2026-02-31\t\t\t\t\nFoo Bar ))))\n-41,45\n \n \t	62.930,69\t\t", "--bank-name", "db"])
            .output()
            .expect("binary should run");

        assert!(!output.status.success());
        assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
        assert_eq!(String::from_utf8(output.stderr).unwrap().trim_end(), "Unable to parse input data. Please check that the correct bank has been selected, and that the input data has been correctly formatted. Call program with --help for more information.");
    }
}