//! Command line arguments for "credit".

use clap::Args;
use sqlx::types::Decimal;

#[derive(Args)]
pub struct CreditArgs {
    /// Short name of the account receiving the credit
    pub account: String,

    /// Value being received
    pub value: Decimal,

    /// Optional note.
    pub note: Option<String>,
}
