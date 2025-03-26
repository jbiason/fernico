use clap::Args;
use sqlx::types::Decimal;

#[derive(Args)]
pub struct TransferArgs {
    /// Short name of the account being debited
    pub debit: String,

    /// Short name of the account being credited
    pub credit: String,

    /// Value being transferred
    pub value: Decimal,

    /// Optional note
    pub note: Option<String>,
}
