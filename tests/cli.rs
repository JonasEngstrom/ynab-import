use std::process::{Command, Stdio};
use std::io::Write;

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ynab-import"))
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
        .args(["2026-06-25\t\t\t\t\nFoo Bar ))))\n-41,45\n \n \t\t62.930,69\t\t", "--bank-name", "db", "--memo", "A short memo"])
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
        .args(["2026-02-31\t\t\t\t\nFoo Bar ))))\n-41,45\n \n \t\t62.930,69\t\t", "--bank-name", "db"])
        .output()
        .expect("binary should run");

    assert!(!output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "");
    assert_eq!(String::from_utf8(output.stderr).unwrap().trim_end(), "Unable to parse input data. Please check that the correct bank has been selected, and that the input data has been correctly formatted. Call program with --help for more information.");
}