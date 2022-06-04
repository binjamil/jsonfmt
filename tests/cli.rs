use assert_cmd::Command;
use std::error::Error;
use std::fs;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "jsonfmt";
const BASIC: &str = "tests/inputs/basic.txt";
const PRODUCT: &str = "tests/inputs/product.txt";
const QUOTES: &str = "tests/inputs/quotes.txt";

// --------------------------------------------------
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

// --------------------------------------------------
fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn basic() -> TestResult {
    run(&[BASIC], "tests/expected/basic.txt.out")
}

#[test]
fn basic_stdin() -> TestResult {
    run_stdin(BASIC, &["-"], "tests/expected/basic.txt.out")
}

#[test]
fn product() -> TestResult {
    run(&[PRODUCT], "tests/expected/product.txt.out")
}

#[test]
fn product_stdin() -> TestResult {
    run_stdin(PRODUCT, &["-"], "tests/expected/product.txt.out")
}

#[test]
fn quotes() -> TestResult {
    run(&[QUOTES], "tests/expected/quotes.txt.out")
}

#[test]
fn quotes_stdin() -> TestResult {
    run_stdin(QUOTES, &["-"], "tests/expected/quotes.txt.out")
}