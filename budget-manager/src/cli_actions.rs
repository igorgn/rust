use chrono::Local;

use crate::{
    errors::CliError,
    helpers::*,
    manager::{BudgetManager, Expense},
    mapper::{BudgetActions, ExpenseCategory, Selectable},
};

pub fn handle_action(action: &BudgetActions, manager: &mut BudgetManager) -> Result<(), CliError> {
    match action {
        BudgetActions::CreateBudget => create_budget(manager),
        BudgetActions::AddExpense => add_expense(manager),
        BudgetActions::EditBudget => edit_budget(manager),
        BudgetActions::ShowTotal => show_total_expenses(manager),
        BudgetActions::DeleteBudget => delete_budget(manager),
        BudgetActions::ListBudgets => list_budgets(manager),
        BudgetActions::ListExpenses => list_expenses(manager),
        BudgetActions::SelectBudget => select_budget(manager),
    }
}

fn select_budget(manager: &mut BudgetManager) -> Result<(), CliError> {
    self::list_budgets(manager)?;
    let input: i64 = get_input("Enter new Budget's id")?;
    manager.switch_budget(input);
    Ok(())
}

fn delete_budget(manager: &mut BudgetManager) -> Result<(), CliError> {
    manager.delete_budget(manager.get_selected_budget_id())?;
    Ok(())
}

fn edit_budget(manager: &mut BudgetManager) -> Result<(), CliError> {
    let input: f64 = get_input("New budget limit: ")?;
    manager.edit_budget(manager.get_selected_budget_id(), input)?;

    Ok(())
}

fn list_budgets(manager: &mut BudgetManager) -> Result<(), CliError> {
    manager.get_budgets()?.iter().for_each(|budget| {
        println!(
            "{} - {}. Limit: {}",
            budget.id, budget.name, budget.budget_limit
        )
    });
    Ok(())
}

pub fn create_budget(manager: &mut BudgetManager) -> Result<(), CliError> {
    let name: String = get_input("Budget's name: ")?;
    let budget_limit: f64 = get_input("Enter budget limit")?;

    if let Ok(id) = manager.create_budget(&name, budget_limit) {
        manager.switch_budget(id);
    }

    Ok(())
}

fn add_expense(manager: &mut BudgetManager) -> Result<(), CliError> {
    let amount: f64 = get_input("Expense amount: ")?;
    let category: ExpenseCategory = get_selection("Select category")?;
    let spent_at = Local::now().format("%Y-%m-%d").to_string();

    let expense = Expense {
        budget_id: manager.get_selected_budget_id(),
        amount,
        category,
        spent_at,
    };
    manager.add_expense(&expense)?;

    Ok(())
}

pub fn show_total_expenses(manager: &mut BudgetManager) -> Result<(), CliError> {
    if let Ok(budget) = manager.get_budget_mut(manager.get_selected_budget_id()) {
        println!(
            "Total expense of {}: {}",
            budget.name,
            budget.total_expenses()
        );
    }
    Ok(())
}

pub fn list_expenses(manager: &mut BudgetManager) -> Result<(), CliError> {
    let expenses = manager.get_expenses(manager.get_selected_budget_id())?;
    expenses
        .iter()
        .for_each(|ex| println!("{} - {} - {}", ex.amount, ex.category.as_str(), ex.spent_at));
    Ok(())
}
