#[derive(Subcommand)]
pub enum Commands {
    /// Account management
    #[command(subcommand)]
    Account(crate::accounts::Actions),

    /// Import movements from a CSV file
    #[command()]
    Import(crate::import::ImportArgs),

    #[command()]
    Export,

    #[command()]
    Credit(CreditArgs),

    #[command()]
    Debit(DebitArgs),

    #[command()]
    Transfer(TransferArgs),
}

// -- Placeholders
#[derive(Args)]
pub struct CreditArgs {
    /// Short name of the account receiving the credit
    pub account: String,

    /// Value being received
    pub value: Decimal,

    /// Optional note.
    pub note: Option<String>,
}

#[derive(Args)]
pub struct DebitArgs {
    /// Short name of the account where the debit is being extracted
    pub account: String,

    /// Value being extracted
    pub value: Decimal,

    /// Optional note.
    pub note: Option<String>,
}

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
