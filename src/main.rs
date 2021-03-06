use anyhow::Error;
use rust_payments_engine::engine::Engine;
use rust_payments_engine::error::PaymentError;
use rust_payments_engine::transaction::Transaction;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::process;

fn run() -> Result<(), Error> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .trim(csv::Trim::All)
        .from_reader(file);
    let mut engine = Engine::new();
    for result in rdr.deserialize() {
        let tx: Transaction = result?;
        println!("{}", tx);
        engine.process_tx(tx);
    }
    for account in engine.accounts_iter() {
        println!("{}", account);
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
