use crate::mapper::Selectable;

use super::manager::*;
use rusqlite::{Connection, Result as SqlResult, params};

pub struct DatabaseManager {
    database: Connection,
}

impl DatabaseManager {
    pub fn new() -> SqlResult<Self> {
        let db = Connection::open("my_db.db3")?; //open_in_memory()?;
        db.execute(
            "
        CREATE TABLE IF NOT EXISTS budgets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            budget_limit REAL NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        ",
            [],
        )?;
        db.execute(
            "
        CREATE TABLE IF NOT EXISTS expenses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            budget_id INTEGER NOT NULL,
            amount REAL NOT NULL,
            category TEXT NOT NULL,
            spent_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (budget_id) REFERENCES budgets(id) ON DELETE CASCADE
        );
        ",
            [],
        )?;
        Ok(Self { database: db })
    }

    pub fn insert_expense(&self, expense: &Expense) -> SqlResult<()> {
        self.database.execute(
            "INSERT INTO expenses (budget_id, amount, category, spent_at) VALUES (?1, ?2, ?3, ?4)",
            params![
                expense.budget_id,
                expense.amount,
                expense.category.as_str(),
                expense.spent_at
            ],
        )?;
        Ok(())
    }

    pub fn get_expenses(&self, budget_id: i64) -> SqlResult<Vec<Expense>> {
        let mut stmt = self.database.prepare(
            "
        SELECT * FROM expenses WHERE budget_id=?1",
        )?;

        stmt.query_map([budget_id], |row| Expense::try_from(row))?
            .collect()
    }

    pub fn insert_budget(&self, budget: &Budget) -> SqlResult<i64> {
        self.database.execute(
            "
        INSERT INTO budgets (name, budget_limit) VALUES (?1, ?2)
        ",
            rusqlite::params![&budget.name, &budget.budget_limit],
        )?;
        Ok(self.database.last_insert_rowid())
    }

    pub fn fetch_budgets(&self) -> SqlResult<Vec<Budget>> {
        let mut stmt = self.database.prepare(
            "
        SELECT * FROM budgets",
        )?;
        // let mut budgets = Vec::new();
        let budgets_db = stmt
            .query_map([], |row| {
                let id = row.get("id")?;
                let b = Budget {
                    id,
                    name: row.get("name")?,
                    budget_limit: row.get("budget_limit")?,
                    expenses: self.get_expenses(id)?,
                };
                // budgets.push(b.clone());
                Ok(b)
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(budgets_db)
    }

    pub fn update_budget(&self, id: i64, new_budget: f64) -> SqlResult<()> {
        self.database.execute(
            "
            UPDATE budgets SET budget_limit = ?1 WHERE id = ?2
            ",
            rusqlite::params![new_budget, id],
        )?;
        Ok(())
    }

    pub fn remove_budget(&self, id: i64) -> SqlResult<()> {
        self.database.execute(
            "
        DELETE FROM budgets WHERE id = ?1",
            [id],
        )?;
        Ok(())
    }
}
