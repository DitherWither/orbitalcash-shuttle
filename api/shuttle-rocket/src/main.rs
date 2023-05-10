use std::path::PathBuf;

use rocket::fs::FileServer;
use shuttle_secrets::SecretStore;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_static_folder::StaticFolder(folder = "dist")] static_folder: PathBuf,
) -> shuttle_rocket::ShuttleRocket {
    // Check if we should serve static files from the secrets.toml file.
    // If the variable isn't set there, just assume that we don't need to serve static files
    let serve_static_files = if let Some(secret) = secret_store.get("SERVE_STATIC_FILES") {
        secret.starts_with("true")
    } else {
        println!("Couldn't get `SERVE_STATIC_FILES` variable from secrets. Assuming `false` as the value");
        false
    };

    let rocket = rocket::build().mount("/api", routes![index]);

    let rocket = if serve_static_files {
        rocket.mount("/", FileServer::from(static_folder))
    } else {
        rocket
    };

    Ok(rocket.into())
}
