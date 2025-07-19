use crate::cli_actions::*;
use crate::errors::CliError;
use crate::helpers::get_selection;
use crate::manager::*;
use crate::mapper::BudgetActions;

pub fn run_cli() -> Result<(), CliError> {
    let mut manager = BudgetManager::new()?;

    loop {
        let selection: BudgetActions = get_selection("Choose action:")?;
        clearscreen::clear().expect("Failed to clear screen");
        if let Err(e) = handle_action(&selection, &mut manager) {
            eprintln!("{e}")
        }
    }
}
