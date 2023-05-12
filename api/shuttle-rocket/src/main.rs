mod routes;

use std::path::PathBuf;

use core::services::ApplicationService;
use rocket::{fs::FileServer, Config};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;

#[macro_use]
extern crate rocket;
#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_static_folder::StaticFolder(folder = "dist")] static_folder: PathBuf,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_rocket::ShuttleRocket {
    // Check if we should serve static files from the secrets.toml file.
    // If the variable isn't set there, just assume that we don't need to serve static files
    let serve_static_files = if let Some(secret) = secret_store.get("SERVE_STATIC_FILES") {
        secret.starts_with("true")
    } else {
        println!("Couldn't get `SERVE_STATIC_FILES` variable from secrets. Assuming `false` as the value");
        false
    };

    // Set a secret key for encrypting cookies
    let secret_key = secret_store
        .get("SECRET_KEY")
        .expect("to get secret key from secret store");

    let config = Config::figment().merge(("secret_key", secret_key));
    let config = Config::from(config);

    let application_service = ApplicationService::new(pool)
        .await
        .expect("to create application service");

    let rocket = rocket::build()
        .mount("/api/users", routes::users::get_routes())
        .mount("/api/auth", routes::auth::get_routes())
        .manage(application_service)
        .configure(config);

    let rocket = if serve_static_files {
        rocket.mount("/", FileServer::from(static_folder))
    } else {
        rocket
    };

    Ok(rocket.into())
}
