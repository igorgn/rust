use chrono::Local;

use crate::{
    errors::CliError,
    helpers::*,
    manager::{BudgetManager, Expense},
    mapper::{BudgetActions, ExpenseCategory, Selectable},
};

pub struct CliContext<'a> {
    pub manager: &'a mut BudgetManager,
    pub current_budget_id_selection: &'a mut i64,
}

pub fn handle_action(action: &BudgetActions, ctx: &mut CliContext) -> Result<(), CliError> {
    match action {
        //TODO rethink commands and output.
        BudgetActions::CreateBudget => create_budget(ctx),
        BudgetActions::AddExpense => add_expense(ctx),
        BudgetActions::EditBudget => edit_budget(ctx),
        BudgetActions::ShowTotal => show_total_expenses(ctx),
        BudgetActions::DeleteBudget => delete_budget(ctx),
        BudgetActions::ListBudgets => list_budgets(ctx),
        BudgetActions::ListExpenses => list_expenses(ctx),
        BudgetActions::SelectBudget => select_budget(ctx),
    }
}

fn select_budget(ctx: &mut CliContext<'_>) -> Result<(), CliError> {
    self::list_budgets(ctx)?;
    let input: i64 = get_input("Enter new Budget's id")?;
    *ctx.current_budget_id_selection = input;
    Ok(())
}

fn delete_budget(ctx: &mut CliContext<'_>) -> Result<(), CliError> {
    ctx.manager
        .delete_budget(*ctx.current_budget_id_selection)?;
    Ok(())
}

fn edit_budget(ctx: &mut CliContext<'_>) -> Result<(), CliError> {
    let input: f64 = get_input("New budget limit: ")?;
    ctx.manager
        .edit_budget(*ctx.current_budget_id_selection, input)?;

    Ok(())
}

fn list_budgets(ctx: &mut CliContext) -> Result<(), CliError> {
    ctx.manager.get_budgets()?.iter().for_each(|budget| {
        println!(
            "{} - {}. Limit: {}",
            budget.id, budget.name, budget.budget_limit
        )
    });
    Ok(())
}

pub fn create_budget(ctx: &mut CliContext) -> Result<(), CliError> {
    let name: String = get_input("Budget's name: ")?;
    let budget_limit: f64 = get_input("Enter budget limit")?;

    if let Ok(id) = ctx.manager.create_budget(&name, budget_limit) {
        *ctx.current_budget_id_selection = id;
    }

    Ok(())
}

fn add_expense(ctx: &mut CliContext) -> Result<(), CliError> {
    let amount: f64 = get_input("Expense amount: ")?;
    let category: ExpenseCategory = get_selection("Select category")?;
    let spent_at = Local::now().format("%Y-%m-%d").to_string();

    let expense = Expense {
        budget_id: *ctx.current_budget_id_selection,
        amount,
        category,
        spent_at,
    };
    ctx.manager.add_expense(&expense)?;

    Ok(())
}

pub fn show_total_expenses(ctx: &mut CliContext) -> Result<(), CliError> {
    if let Ok(budget) = ctx.manager.get_budget_mut(*ctx.current_budget_id_selection) {
        println!(
            "Total expense of {}: {}",
            budget.name,
            budget.total_expenses()
        );
    }
    Ok(())
}

pub fn list_expenses(ctx: &mut CliContext<'_>) -> Result<(), CliError> {
    let expenses = ctx.manager.get_expenses(*ctx.current_budget_id_selection)?;
    expenses
        .iter()
        .for_each(|ex| println!("{} - {} - {}", ex.amount, ex.category.as_str(), ex.spent_at));
    Ok(())
}
