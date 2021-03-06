use anyhow::Error;
use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use rust_payments_engine::engine::Engine;
use rust_payments_engine::error::PaymentError;
use rust_payments_engine::transaction::Transaction;
use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io;
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
        engine.process_tx(tx);
    }

    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.write_record(&["client", "available", "held", "total", "locked"])?;
    for account in engine.accounts_iter() {
        //println!("{}", account);
        wtr.write_record(&[
            account.client().to_string(),
            amount_to_str(account.available)?,
            amount_to_str(account.held)?,
            amount_to_str(account.total())?,
            account.locked.to_string(),
        ])?;
    }
    wtr.flush()?;
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

fn amount_to_str(amount: u64) -> Result<String, Error> {
    let dec = match Decimal::from_u64(amount) {
        Some(u) => u,
        None => {
            return Err(PaymentError::SerializeError(
                "could not serialize amount to decimal".to_string(),
            )
            .into())
        }
    };
    let dec = dec.checked_div(Decimal::new(10000, 0)).unwrap();
    Ok(dec.to_string())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
