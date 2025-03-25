//! Import files into the database

mod args;
use std::str::FromStr;

pub use args::ImportArgs;
use sqlx::{types::Decimal, SqlitePool};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

use crate::accounts::Account;

pub async fn run(args: &ImportArgs, pool: &SqlitePool) {
    let file = File::open(&args.filename).await.unwrap();
    let buffer = BufReader::new(file);
    let mut lines = buffer.lines();

    let Ok(Some(header)) = lines.next_line().await else {
        println!("Missing header");
        return;
    };

    // Formats:
    // account: short,name
    // entry: short,date,credit,debit,note
    let mut frags = header.split(',');
    let _field1 = frags.next();
    let field2 = frags.next();

    if field2 == Some("name") {
        import_accounts(&mut lines, pool).await;
    } else if field2 == Some("date") {
        import_entries(&mut lines).await;
    } else {
        println!("Unknown format.");
        println!("Value formats are:");
        println!("Import accounts: short,name");
        println!("Import entries: short,date,credit,debit,note");
    }
}

async fn import_accounts(lines: &mut tokio::io::Lines<BufReader<File>>, pool: &SqlitePool) {
    let mut count = 1;
    while let Ok(Some(line)) = lines.next_line().await {
        count += 1;
        let mut frags = line.split(',');
        let Some(short) = frags.next() else {
            println!("Line {count} does not have a short name; ignoring the line");
            continue;
        };

        let Some(name) = frags.next() else {
            println!("Line {count} does not have a name; ignoring the line");
            continue;
        };

        let account = Account::new(short, name);
        account.save(pool).await;
    }
}

async fn import_entries(lines: &mut tokio::io::Lines<BufReader<File>>) {
    let mut count = 1;
    while let Ok(Some(line)) = lines.next_line().await {
        count += 1;
        let mut frags = line.split(",");

        let Some(account) = frags.next() else {
            println!("Line {count} does not have an account information; ignoring the line");
            continue;
        };

        let Some(date) = frags.next() else {
            println!("Line {count} does not have a date; ignoring the line");
            continue;
        };

        let credit = frags.next().unwrap_or("0");
        let debit = frags.next().unwrap_or("0");
        let description = frags.next().unwrap_or("");

        let Ok(credit) = Decimal::from_str(credit) else {
            println!("Line {count} has an invalid value for credit ({credit}); ignoring the line");
            continue;
        };

        let Ok(debit) = Decimal::from_str(debit) else {
            println!("Line {count} has an invalid value for debit ({debit}); ignoring the line");
            continue;
        };

        if debit + credit == Decimal::new(0, 0) {
            println!("Line {count} does not any credit or debit value; ignoring the line");
            continue;
        }
    }
}
