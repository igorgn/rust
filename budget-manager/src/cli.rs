use crate::cli_actions::*;
use crate::errors::CliError;
use crate::helpers::get_selection;
use crate::manager::*;
use crate::mapper::BudgetActions;

pub fn run_cli() -> Result<(), CliError> {
    let mut manager = BudgetManager::new()?;
    let mut current_budget_id_selection: i64 = 1;
    let mut ctx = CliContext {
        manager: &mut manager,
        current_budget_id_selection: &mut current_budget_id_selection,
    };

    loop {
        let selection: BudgetActions = get_selection("Choose action:")?;
        clearscreen::clear().expect("Failed to clear screen");
        if let Err(e) = handle_action(&selection, &mut ctx) {
            eprintln!("{e}")
        }
    }
}
