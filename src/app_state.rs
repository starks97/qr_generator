use crate::config_secrets::Config;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub secrets: Config,
}

impl AppState {
    pub fn new(db: PgPool, secrets: Config) -> Self {
        Self { db, secrets }
    }
}
