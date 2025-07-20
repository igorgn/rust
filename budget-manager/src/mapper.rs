use rusqlite::types::{FromSql, FromSqlError, ValueRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum BudgetActions {
    CreateBudget,
    AddExpense,
    EditBudget,
    ShowTotal,
    DeleteBudget,
    ListBudgets,
    ListExpenses,
    SelectBudget,
}

pub trait Selectable: Sized {
    // type Item;
    fn all() -> &'static [Self];
    fn as_str(&self) -> &'static str;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ExpenseCategory {
    Housing,
    Transportation,
    Food,
    Healthcare,
    Entertainment,
    PersonalCare,
    Miscellaneous,
}

impl Selectable for BudgetActions {
    fn all() -> &'static [BudgetActions] {
        &[
            BudgetActions::CreateBudget,
            BudgetActions::AddExpense,
            BudgetActions::EditBudget,
            BudgetActions::ShowTotal,
            BudgetActions::DeleteBudget,
            BudgetActions::ListBudgets,
            BudgetActions::ListExpenses,
            BudgetActions::SelectBudget,
        ]
    }

    fn as_str(&self) -> &'static str {
        match self {
            BudgetActions::CreateBudget => "Create budget",
            BudgetActions::AddExpense => "Add expense",
            BudgetActions::EditBudget => "Edit budget",
            BudgetActions::ShowTotal => "Show total",
            BudgetActions::DeleteBudget => "Delete budget",
            BudgetActions::ListBudgets => "List budgets",
            BudgetActions::ListExpenses => "List expenses",
            BudgetActions::SelectBudget => "Select Budget",
        }
    }
}

impl Selectable for ExpenseCategory {
    fn all() -> &'static [Self] {
        &[
            ExpenseCategory::Housing,
            ExpenseCategory::Transportation,
            ExpenseCategory::Food,
            ExpenseCategory::Healthcare,
            ExpenseCategory::Entertainment,
            ExpenseCategory::PersonalCare,
            ExpenseCategory::Miscellaneous,
        ]
    }

    fn as_str(&self) -> &'static str {
        match self {
            ExpenseCategory::Housing => "Housing",
            ExpenseCategory::Transportation => "Transportation",
            ExpenseCategory::Food => "Food",
            ExpenseCategory::Healthcare => "Healthcare",
            ExpenseCategory::Entertainment => "Entertainment",
            ExpenseCategory::PersonalCare => "Personal care",
            ExpenseCategory::Miscellaneous => "Miscellaneous",
        }
    }
}

impl FromSql for ExpenseCategory {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            ValueRef::Text(text) => match text {
                b"Housing" => Ok(ExpenseCategory::Housing),
                b"Transportation" => Ok(ExpenseCategory::Transportation),
                b"Food" => Ok(ExpenseCategory::Food),
                b"Healthcare" => Ok(ExpenseCategory::Healthcare),
                b"Entertainment" => Ok(ExpenseCategory::Entertainment),
                b"Personal care" => Ok(ExpenseCategory::PersonalCare),
                b"Miscellaneous" => Ok(ExpenseCategory::Miscellaneous),
                _ => Err(rusqlite::types::FromSqlError::InvalidType),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}
