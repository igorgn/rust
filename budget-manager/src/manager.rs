use super::db::DatabaseManager;
use crate::{errors::BudgetErrors, mapper::ExpenseCategory};
use rusqlite::{Result, Row};

trait SqlResultExt<T> {
    fn to_budget_err(self) -> Result<T, BudgetErrors>;
}

pub struct BudgetManager {
    db: DatabaseManager,
    budgets: Vec<Budget>,
}

#[derive(Debug, Clone)]
pub struct Budget {
    pub id: i64,
    pub name: String,
    pub budget_limit: f64,
    pub expenses: Vec<Expense>,
}
#[derive(Debug, Clone)]
pub struct Expense {
    pub budget_id: i64,
    pub amount: f64,
    pub category: ExpenseCategory,
    pub spent_at: String,
}

impl<T> SqlResultExt<T> for rusqlite::Result<T> {
    fn to_budget_err(self) -> rusqlite::Result<T, BudgetErrors> {
        self.map_err(|e| BudgetErrors::SqlError(e))
    }
}

impl<'a> std::convert::TryFrom<&'a Row<'a>> for Expense {
    type Error = rusqlite::Error;

    fn try_from(row: &'a Row) -> std::result::Result<Self, Self::Error> {
        Ok(Expense {
            budget_id: row.get("budget_id")?,
            amount: row.get("amount")?,
            category: row.get("category")?,
            spent_at: row.get("spent_at")?,
        })
    }
}

impl BudgetManager {
    pub fn new() -> Result<Self, BudgetErrors> {
        let db = match DatabaseManager::new() {
            Ok(db) => db,
            Err(_) => {
                eprintln!("Failed to initialize database");
                std::process::exit(1)
            }
        };
        // TODO Fix
        let budgets = db.fetch_budgets()?;

        Ok(Self { db, budgets })
    }

    pub fn create_budget(&mut self, name: &str, budget_limit: f64) -> Result<i64, BudgetErrors> {
        if budget_limit < 0.0 {
            return Err(BudgetErrors::InvalidBudgetLimit);
        }

        let temp_budget = Budget {
            id: 0,
            name: name.to_string(),
            budget_limit,
            expenses: Vec::new(),
        };

        let id = self.db.insert_budget(&temp_budget)?;
        let new_budget = Budget { id, ..temp_budget };

        self.budgets.push(new_budget);

        Ok(id)
    }
    pub fn edit_budget(&mut self, id: i64, new_budget: f64) -> Result<(), BudgetErrors> {
        self.db.update_budget(id, new_budget)?;
        let budget_mut = self.get_budget_mut(id)?;
        budget_mut.budget_limit = new_budget;

        Ok(())
    }

    pub fn get_budgets(&self) -> Result<Vec<Budget>, BudgetErrors> {
        self.db.fetch_budgets().to_budget_err()
    }

    pub fn delete_budget(&mut self, id: i64) -> Result<(), BudgetErrors> {
        self.db.remove_budget(id).to_budget_err()?;
        if let Some(pos) = self.budgets.iter().position(|budget| budget.id == id) {
            self.budgets.remove(pos);
        }
        Ok(())
    }

    pub fn add_expense(&mut self, expense: &Expense) -> Result<(), BudgetErrors> {
        let bud_name: String;
        {
            let budget_mut = self.get_budget_mut(expense.budget_id)?;
            budget_mut.add_expense(expense)?;
            bud_name = budget_mut.name.to_string();
        }
        self.db.insert_expense(expense).to_budget_err()?;
        println!("{}: Expense added successfuly!", bud_name);

        Ok(())
    }

    pub fn get_expenses(&self, budget_id: i64) -> Result<Vec<Expense>, BudgetErrors> {
        self.db.get_expenses(budget_id).to_budget_err()
    }

    pub fn get_budget_mut(&mut self, id: i64) -> Result<&mut Budget, BudgetErrors> {
        match self.budgets.iter_mut().find(|b| b.id == id) {
            Some(budget) => Ok(budget),
            None => Err(BudgetErrors::BudgetNotFound(id)),
        }
    }
}

impl Budget {
    pub fn add_expense(&mut self, expense: &Expense) -> Result<(), BudgetErrors> {
        if self.if_enough_money(expense.amount) {
            self.expenses.push(expense.clone());
        } else {
            return Err(BudgetErrors::NotEnoughMoney);
        }
        Ok(())
    }

    pub fn total_expenses(&self) -> f64 {
        self.expenses.iter().map(|expense| expense.amount).sum()
    }

    pub fn if_enough_money(&self, amount: f64) -> bool {
        self.budget_limit >= amount
    }
}

#[cfg(test)]
mod tests {

    use chrono::Local;

    use super::*;

    struct TestBudgetManager {
        bm: BudgetManager,
        id: i64,
    }

    fn init() -> TestBudgetManager {
        let mut manager = BudgetManager::new().unwrap();
        let id = manager.create_budget("test budget", 50.0).unwrap();
        TestBudgetManager { bm: manager, id }
    }

    #[test]
    fn create_budget() {
        let mut manager = BudgetManager::new().unwrap();
        let budget_id = manager.create_budget("test_budget", 50.0).unwrap();
        assert_eq!(1, budget_id);
    }

    #[test]
    fn get_budget_limit() {
        let mut manager = init();
        assert_eq!(
            50.0,
            manager.bm.get_budget_mut(manager.id).unwrap().budget_limit
        );
    }
    #[test]
    fn edit_budget() {
        let mut manager = init();
        manager.bm.edit_budget(manager.id, 100.0).unwrap();
    }

    #[test]
    fn add_expense() {
        let mut manager = init();
        let expense = Expense {
            budget_id: 1,
            amount: 10.0,
            category: ExpenseCategory::Entertainment,
            spent_at: Local::now().format("%Y-%m-%d").to_string(),
        };
        manager
            .bm
            .get_budget_mut(1)
            .unwrap()
            .add_expense(&expense)
            .unwrap();
    }
}
