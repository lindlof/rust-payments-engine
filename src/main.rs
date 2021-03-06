use anyhow::Error;
use rust_payments_engine::error::PaymentError;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::process;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Transaction {
    #[serde(rename = "type")]
    txtype: String,
    client: String,
    tx: String,
    amount: String,
}

fn run() -> Result<(), Error> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .trim(csv::Trim::All)
        .from_reader(file);
    for result in rdr.deserialize() {
        let record: Transaction = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn get_first_arg() -> Result<OsString, Error> {
    match env::args_os().nth(1) {
        None => {
            Err(PaymentError::InputError("expected 1 argument, but got none".to_string()).into())
        }
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
