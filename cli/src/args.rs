/*
   Fernico - Finance Storage
   Copyright (C) 2021  Julio Biason

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Affero General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Affero General Public License for more details.

   You should have received a copy of the GNU Affero General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use clap::crate_authors;
use clap::crate_description;
use clap::crate_name;
use clap::crate_version;
use clap::App;
use clap::Arg;
use clap::ArgMatches;
use clap::SubCommand;
use rust_decimal::prelude::*;

use crate::commands::add_expense::AddExpense;
use crate::commands::interface::Command;

/// Errors in the CLI parsing
pub enum CliErrors {
    /// Request to set a value without a value
    ValueIsRequired,
    /// There is no such command
    NoSuchCommand,
}

pub(crate) fn cli() -> Result<Box<dyn Command>, CliErrors> {
    let interface = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("expense")
                .about("Expense management")
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add a new expense")
                        .arg(Arg::with_name("date")
                            .short("d")
                            .long("date")
                            .help("Date for the expense")
                            .long_help("Required only if you want to register an expense not in the current date"))
                        .arg(Arg::with_name("value").help("Amount spent on expense"))
                        .arg(Arg::with_name("tags").help("Tags for the expense").multiple(true))
                ),
        );
    let matches = interface.get_matches();
    match matches.subcommand() {
        ("expense", Some(expense_arguments)) => {
            parse_expenses(expense_arguments)
        }
        _ => Err(CliErrors::NoSuchCommand),
    }
}

fn parse_expenses(args: &ArgMatches) -> Result<Box<dyn Command>, CliErrors> {
    match args.subcommand() {
        ("add", Some(params)) => {
            let value =
                params.value_of("value").ok_or(CliErrors::ValueIsRequired)?;
            let tags = params.values_of("tags").map_or_else(
                || vec![],                                       /* else */
                |values| values.map(|val| val.into()).collect(), /* map */
            );
            let date = params.value_of("date").unwrap_or("Today");
            log::debug!(
                "Add expense on {:?}, of {:?}, with tags {:?}",
                date,
                value,
                tags
            );
            Ok(Box::new(AddExpense::new(
                Decimal::from_str(value).unwrap(),
                tags,
            )))
        }
        _ => Err(CliErrors::NoSuchCommand),
    }
}
