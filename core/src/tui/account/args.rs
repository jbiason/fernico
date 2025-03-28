use clap::{Args, Subcommand};

use crate::models::account::Account;

#[derive(Subcommand)]
pub enum AccountArgs {
    /// Add a new account
    Add(AddArgs),

    /// Remove an account
    Del(DelArgs),

    /// List the accounts
    List,
}

#[derive(Args)]
pub struct AddArgs {
    /// The short name/identifier for the account
    pub short: String,

    /// The full name for the account
    pub name: String,
}

#[derive(Args)]
pub struct DelArgs {
    /// Identifier of the account to be deleted
    pub short: String,
}

impl From<AddArgs> for Account {
    fn from(value: AddArgs) -> Self {
        Account::new(&value.short, &value.name)
    }
}
