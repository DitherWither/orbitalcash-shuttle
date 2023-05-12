use app_core::services::{user_service::UserGetError, ApplicationService};

use rocket::{
    http::Status,
    response::{content::RawJson, status},
    serde::json::serde_json::{self, json},
    Route, State,
};

pub fn get_routes() -> Vec<Route> {
    routes![get_all, get_by_id]
}

#[get("/")]
async fn get_all(app_service: &State<ApplicationService>) -> status::Custom<RawJson<String>> {
    let users = app_service.users.get_all().await;

    match users {
        // TODO: remove this expect
        Ok(e) => status::Custom(
            Status::Ok,
            RawJson(json!({
                "status": "success",
                "users": e
            }).to_string()),
        ),
        Err(e) => {
            let json = serde_json::json!(
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

#[get("/<user_id>")]
async fn get_by_id(
    app_service: &State<ApplicationService>,
    user_id: i32,
) -> status::Custom<RawJson<String>> {
    let user = app_service.users.get_by_id(user_id).await;

    match user {
        Ok(e) => status::Custom(
            Status::Ok,
            RawJson(json!({
                "status": "success",
                "user": e
            }).to_string()),
        ),
        Err(UserGetError::DoesNotExist(id)) => status::Custom(
            Status::NotFound,
            RawJson(
                serde_json::json!({
                    "status": "error",
                    "error_type": "not_found",
                    "error": format!("User with id `{}` does not exist", id)
                })
                .to_string(),
            ),
        ),
        Err(UserGetError::DatabaseError(e)) => {
            let json = serde_json::json!(
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
