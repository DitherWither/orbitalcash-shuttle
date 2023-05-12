use app_core::services::{
    user_service::{LoginBody, LoginError, RegisterBody, RegisterError, UserGetError},
    ApplicationService,
};

use rocket::{
    http::{Cookie, CookieJar, Status},
    response::{content::RawJson, status},
    serde::json::{
        serde_json::{self, json},
        Json,
    },
    Route, State,
};

use crate::guards::CurrentUser;

pub fn get_routes() -> Vec<Route> {
    routes![login, logout, register, get_user_id, get_current_user]
}

#[post("/login", data = "<body>")]
pub async fn login(
    cookies: &CookieJar<'_>,
    body: Json<LoginBody>,
    app_service: &State<ApplicationService>,
) -> status::Custom<RawJson<String>> {
    match app_service.users.login(&body).await {
        Ok(user_id) => {
            cookies.add_private(Cookie::new("user_id", user_id.to_string()));
            status::Custom(
                Status::Ok,
                RawJson(json!({ "status": "success" }).to_string()),
            )
        }
        Err(e) => match e {
            LoginError::InvalidUser => status::Custom(
                Status::BadRequest,
                RawJson(
                    serde_json::json!({
                        "status": "error",
                        "error_type": "invalid_user",
                        "error": "Invalid user"
                    })
                    .to_string(),
                ),
            ),
            LoginError::DatabaseError(e) => status::Custom(
                Status::InternalServerError,
                RawJson(
                    serde_json::json!({
                        "status": "error",
                        "error_type": "database_error",
                        "error": e.to_string()
                    })
                    .to_string(),
                ),
            ),
            LoginError::InvalidPassword => status::Custom(
                Status::BadRequest,
                RawJson(
                    serde_json::json!({
                        "status": "error",
                        "error_type": "invalid_password",
                        "error": "Invalid password"
                    })
                    .to_string(),
                ),
            ),
            LoginError::PasswordHashError(e) => status::Custom(
                Status::InternalServerError,
                RawJson(
                    serde_json::json!({
                        "status": "error",
                        "error_type": "password_hash_error",
                        "error": e.to_string()
                    })
                    .to_string(),
                ),
            ),
        },
    }
}

#[post("/logout")]
pub async fn logout(cookies: &CookieJar<'_>) -> status::Custom<RawJson<String>> {
    cookies.remove_private(Cookie::named("user_id"));
    status::Custom(
        Status::Ok,
        RawJson(json!({ "status": "success" }).to_string()),
    )
}

#[post("/register", data = "<body>")]
pub async fn register(
    cookies: &CookieJar<'_>,
    body: Json<RegisterBody>,
    app_service: &State<ApplicationService>,
) -> status::Custom<RawJson<String>> {
    match app_service.users.register(&body).await {
        Ok(user_id) => {
            cookies.add_private(Cookie::new("user_id", user_id.to_string()));
            status::Custom(
                Status::Ok,
                RawJson(json!({ "status": "success" }).to_string()),
            )
        }
        Err(e) => match e {
            RegisterError::EmailAlreadyInUse => status::Custom(
                Status::BadRequest,
                RawJson(
                    serde_json::json!({
                        "status": "error",
                        "error_type": "email_already_in_use",
                        "error": "Email is already in use"
                    })
                    .to_string(),
                ),
            ),
            RegisterError::DatabaseError(e) => status::Custom(
                Status::InternalServerError,
                RawJson(
                    serde_json::json!({
                        "status": "error",
                        "error_type": "database_error",
                        "error": e.to_string()
                    })
                    .to_string(),
                ),
            ),
            RegisterError::PasswordHashError(e) => status::Custom(
                Status::InternalServerError,
                RawJson(
                    serde_json::json!({
                        "status": "error",
                        "error_type": "password_hash_error",
                        "error": e.to_string()
                    })
                    .to_string(),
                ),
            ),
        },
    }
}

#[get("/user_id")]
pub async fn get_user_id(cookies: &CookieJar<'_>) -> status::Custom<RawJson<String>> {
    match cookies.get_private("user_id") {
        Some(cookie) => status::Custom(
            Status::Ok,
            RawJson(json!({ "user_id": cookie.value() }).to_string()),
        ),
        None => status::Custom(
            Status::BadRequest,
            RawJson(
                serde_json::json!({
                    "status": "error",
                    "error_type": "not_logged_in",
                    "error": "Not logged in"
                })
                .to_string(),
            ),
        ),
    }
}

#[get("/current_user")]
pub async fn get_current_user(
    user: CurrentUser,
    app_service: &State<ApplicationService>,
) -> status::Custom<RawJson<String>> {
    match app_service.users.get_by_id(user.0).await {
        Ok(user) => status::Custom(
            Status::Ok,
            RawJson(serde_json::json!({ "status": "success", "user": user }).to_string()),
        ),
        Err(e) => {
            match e {
                UserGetError::DatabaseError(e) => status::Custom(
                    Status::InternalServerError,
                    RawJson(
                        serde_json::json!({
                            "status": "error",
                            "error_type": "database_error",
                            "error": e.to_string()
                        })
                        .to_string(),
                    ),
                ),
                UserGetError::DoesNotExist(e) =>
                    status::Custom(
                        Status::InternalServerError,
                        RawJson(
                            serde_json::json!({
                                "status": "error",
                                "error_type": "user does not exist",
                                "error": format!("The currently logged in user with id {e} does not exist. How did this happen?"),
                            })
                            .to_string()))
                
            }
        }
    }
}
