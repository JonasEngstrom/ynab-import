[![Build and Test](https://github.com/JonasEngstrom/ynab-import/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/JonasEngstrom/ynab-import/actions/workflows/rust.yml)
[![Crates.io Version](https://img.shields.io/crates/v/ynab-import)](https://crates.io/crates/ynab-import)
[![codecov](https://codecov.io/gh/JonasEngstrom/ynab-import/graph/badge.svg?token=KIYAPII5VE)](https://codecov.io/gh/JonasEngstrom/ynab-import)

# YNAB Import

Format copied bank statements as CSV for import to [YNAB](https://www.ynab.com/).

This crate was made to speed up data entry into YNAB. It is built on a python script that the author has used for several years to transform data copied directly from the Swedish branch of Danske Bank’s web interface into CSV format for import into YNAB. Danske Bank is also the first bank supported by this crate.

## Usage

Start by selecting and copying the bank statement you want to import. On macOS you can then pipe the data to this crate by using `pbpaste` in the termina. On other systems the command might be different, but you can also paste to a file that you then pipe into the program.

```bash
pbpaste | ynab-import --bank danske-bank > ynab_file.csv
```

> [!NOTE]
> Instead of piping, the input data can be pasted as an argument after any options provided.

## Supported Banks

To pick a bank, use the option `--bank` or `-b`. Banks also have abbreviations.

|Bank|Option|Abbreviation|
|-|-|-|
|Danske Bank, Sverige Filial|`danske-bank`|`db`|

## Getting Help

To get instructions on how to use the program, run it with the `--help` or `-h` options.

```bash
ynab-import --help
```