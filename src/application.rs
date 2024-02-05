use crate::{
    result::{AppError, AppResult},
    settings::{database_settings::DatabaseSettings, Settings},
};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

pub async fn build_app(settings: Settings) -> AppResult<()> {
    connect_db(&settings.database).await
}

async fn connect_db(settings: &DatabaseSettings) -> AppResult<()> {
    // Connect to the server
    let db = Surreal::new::<Ws>(&settings.url).await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: &settings.username,
        password: &settings.password,
    })
    .await
    .map_err(|e| AppError::Db { source: e })
    .map(|_| ())
}
