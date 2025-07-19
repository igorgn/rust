use rocket::{data, http::Status, serde::json::{self, Json}, tokio::sync::Mutex, Build, Rocket, State};

use crate::manager::{Budget, BudgetManager};

#[get("/budgets")]
pub async fn get_budgets(manager: &State<Mutex<BudgetManager>>) -> Result<Json<Vec<Budget>>, Status> {
    let manager = manager.lock().await;
    let budgets = manager
        .get_budgets()
        .map_err(|_| Status::InternalServerError)?;
    Ok(Json(budgets))
}

//TODO move it to manager ... and split manager in smaller files
#[derive(serde::Deserialize)]
pub struct NewBudget {
    name: String,
    budget_limit: f64
}
#[post("/", data = "<new_budget>")]
pub async fn create_budget(manager: &State<Mutex<BudgetManager>>, new_budget: Json<NewBudget>) -> Result<(), Status>{
    let mut manager = manager.lock().await;
    manager.create_budget(&new_budget.name, new_budget.budget_limit).map_err(|_| Status::InternalServerError)?;
    Ok(())

}
pub fn build_rocket() -> Rocket<Build> {
    let manager = Mutex::new(BudgetManager::new().unwrap());
    rocket::build()
        .manage(manager)
        .mount("/", routes![get_budgets, create_budget])
}
