use thiserror::Error;

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Budget error: {0}")]
    BudgetError(#[from] BudgetErrors),
    #[error("Input error: {0}")]
    InputError(#[from] dialoguer::Error),
}
#[derive(Debug, Error)]
pub enum BudgetErrors {
    #[error("Database error: {0}")]
    SqlError(#[from] rusqlite::Error),
    #[error("Budget not found {0}")]
    BudgetNotFound(i64),
    #[error("Invalid budget limit")]
    InvalidBudgetLimit,
    #[error("Over budget limit")]
    NotEnoughMoney,
}
