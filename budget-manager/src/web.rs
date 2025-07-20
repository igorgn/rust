use rocket::{
    Build, Rocket, State,
    http::Status,
    serde::{Deserialize, json::Json},
    tokio::sync::Mutex,
};

use crate::manager::{Budget, BudgetManager, Expense};

type SharedBudgetManager = State<Mutex<BudgetManager>>;

#[derive(Deserialize)]
pub struct NewBudget {
    name: String,
    budget_limit: f64,
}

#[get("/expenses/<id>")]
pub async fn get_expenses(
    manager: &SharedBudgetManager,
    id: i64,
) -> Result<Json<Vec<Expense>>, Status> {
    let manager = manager.lock().await;
    let expenses = manager
        .get_expenses(id)
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(expenses))
}

#[post("/expenses/<_id>", data = "<expense>")]
pub async fn add_expense(
    manager: &SharedBudgetManager,
    _id: i64,
    expense: Json<Expense>,
) -> Result<(), Status> {
    let mut manager = manager.lock().await;
    manager
        .add_expense(&expense)
        .map_err(|_| Status::InternalServerError)?;
    Ok(())
}

#[get("/budgets")]
pub async fn get_budgets(manager: &SharedBudgetManager) -> Result<Json<Vec<Budget>>, Status> {
    let manager = manager.lock().await;
    let budgets = manager
        .get_budgets()
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(budgets))
}

#[post("/budgets", data = "<new_budget>")]
pub async fn create_budget(
    manager: &SharedBudgetManager,
    new_budget: Json<NewBudget>,
) -> Result<(), Status> {
    let mut manager = manager.lock().await;
    manager
        .create_budget(&new_budget.name, new_budget.budget_limit)
        .map_err(|_| Status::InternalServerError)?;
    Ok(())
}
pub fn build_rocket() -> Rocket<Build> {
    let manager = Mutex::new(BudgetManager::new().unwrap());
    rocket::build().manage(manager).mount(
        "/",
        routes![get_budgets, create_budget, get_expenses, add_expense],
    )
}
