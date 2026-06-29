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