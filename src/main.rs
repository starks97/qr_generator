use actix_web::{web::Data, App, HttpServer};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use qr_prototype::{
    api::route_handler::config_handler, app_state::AppState, config_secrets,
    custom_error::CustomError,
};

use tracing::{info, warn};

use tracing_subscriber;

#[actix_web::main]
async fn main() -> Result<(), CustomError> {
    let config_data = config_secrets::Config::init();

    tracing_subscriber::fmt::init();

    let database_url = config_data.database_url.to_owned();
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .map_err(|err| {
            warn!("Failed to connect to the database: {:?}", err);
            CustomError::DataBaseError(err)
        })?;

    info!("✅ Connection to the db is successful");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|err| {
            warn!("Failed to connect to run migration: {:?}", err);
            CustomError::OtherError(format!("error: {:?}", err))
        })?;

    info!("Migrations ran successfully");
    info!("Server started successfully 🚀!");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                db: pool.clone(),
                secrets: config_data.clone(),
            }))
            .configure(|ctx| config_handler(ctx, &config_data))
    })
    .bind("127.0.0.1:8080")
    .map_err(|err| {
        warn!("Failed to bind server: {:?}", err);
        CustomError::OtherError(err.to_string())
    })?
    .run()
    .await
    .map_err(|err| {
        warn!("Failed to run the server: {:?}", err);
        CustomError::OtherError(err.to_string())
    })?;

    Ok(())
}
