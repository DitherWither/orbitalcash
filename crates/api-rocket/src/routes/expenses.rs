use db::services::{expense_service::CreateExpense, ApplicationService};
use rocket::{
    http::Status,
    response::{content::RawJson, status},
    serde::json::{serde_json::json, Json},
    Route, State, routes, get, delete, post, put,
};

use crate::guards::CurrentUser;

pub fn get_routes() -> Vec<Route> {
    routes![get_all, get_by_id, delete, create, update]
}

// TODO: Seperate between forbidden and database error

#[get("/")]
async fn get_all(
    user: CurrentUser,
    app_service: &State<ApplicationService>,
) -> status::Custom<RawJson<String>> {
    match app_service.expenses.get_all(user.0).await {
        Ok(e) => status::Custom(
            Status::Ok,
            RawJson(
                json!({
                    "status": "success",
                    "expenses": e
                })
                .to_string(),
            ),
        ),
        Err(e) => {
            let json = json!(
                {
                    "status": "error",
                    "error_type": "database_error",
                    "error": e.to_string()
                }
            );
            status::Custom(Status::InternalServerError, RawJson(json.to_string()))
        }
    }
}

#[get("/<expense_id>")]
async fn get_by_id(
    user: CurrentUser,
    expense_id: i32,
    app_service: &State<ApplicationService>,
) -> status::Custom<RawJson<String>> {
    match app_service
        .expenses
        .get_by_id_checked(expense_id, user.0)
        .await
    {
        Ok(Some(e)) => status::Custom(
            Status::Ok,
            RawJson(
                json!({
                    "status": "success",
                    "expense": e
                })
                .to_string(),
            ),
        ),
        Ok(None) => status::Custom(
            Status::NotFound,
            RawJson(
                json!({
                    "status": "error",
                    "error_type": "not_found",
                    "error": format!("Expense with id `{}` does not exist", expense_id)
                })
                .to_string(),
            ),
        ),
        Err(e) => {
            let json = json!(
                {
                    "status": "error",
                    "error_type": "database_error",
                    "error": e.to_string()
                }
            );
            status::Custom(Status::InternalServerError, RawJson(json.to_string()))
        }
    }
}

#[delete("/<expense_id>")]
async fn delete(
    user: CurrentUser,
    expense_id: i32,
    app_service: &State<ApplicationService>,
) -> status::Custom<RawJson<String>> {
    match app_service
        .expenses
        .delete_by_id_checked(expense_id, user.0)
        .await
    {
        Ok(e) => status::Custom(
            Status::Ok,
            RawJson(
                json!({
                    "status": "success"
                })
                .to_string(),
            ),
        ),
        Err(e) => {
            let json = json!(
                {
                    "status": "error",
                    "error_type": "database_error",
                    "error": e.to_string()
                }
            );
            status::Custom(Status::InternalServerError, RawJson(json.to_string()))
        }
    }
}

#[post("/", data = "<expense>")]
async fn create(
    user: CurrentUser,
    expense: Json<CreateExpense>,
    app_service: &State<ApplicationService>,
) -> status::Custom<RawJson<String>> {
    let expense = expense.into_inner();
    if expense.user_id != user.0 {
        return status::Custom(
            Status::Forbidden,
            RawJson(
                json!({
                    "status": "error",
                    "error_type": "forbidden",
                    "error": "You can only create expenses for yourself"
                })
                .to_string(),
            ),
        );
    };

    match app_service.expenses.create(expense).await {
        Ok(e) => status::Custom(
            Status::Ok,
            RawJson(
                json!({
                    "status": "success",
                    "expense_id": e
                })
                .to_string(),
            ),
        ),
        Err(e) => {
            let json = json!(
                {
                    "status": "error",
                    "error_type": "database_error",
                    "error": e.to_string()
                }
            );
            status::Custom(Status::InternalServerError, RawJson(json.to_string()))
        }
    }
}

#[put("/<expense_id>", data = "<expense>")]
async fn update(
    expense_id: i32,
    expense: Json<CreateExpense>,
    user: CurrentUser,
    app_service: &State<ApplicationService>,
) -> status::Custom<RawJson<String>> {
    match app_service
        .expenses
        .update(expense.into_inner(), expense_id, user.0)
        .await
    {
        Ok(e) => status::Custom(
            Status::Ok,
            RawJson(
                json!({
                    "status": "success",
                    "expense_id": e
                })
                .to_string(),
            ),
        ),
        Err(e) => {
            let json = json!(
                {
                    "status": "error",
                    "error_type": "database_error",
                    "error": e.to_string()
                }
            );
            status::Custom(Status::InternalServerError, RawJson(json.to_string()))
        }
    }
}
