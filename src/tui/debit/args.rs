use clap::Args;
use sqlx::types::Decimal;

#[derive(Args)]
pub struct DebitArgs {
    /// Short name of the account where the debit is being extracted
    pub account: String,

    /// Value being extracted
    pub value: Decimal,

    /// Optional note.
    pub note: Option<String>,
}
