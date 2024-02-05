use crate::{
    result::{AppError, AppResult},
    settings::Settings,
};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

pub async fn build_app(settings: Settings) -> AppResult<()> {
    connect_db(&settings.database.username, &settings.database.password).await
}

async fn connect_db(username: &str, password: &str) -> AppResult<()> {
    // Connect to the server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: username,
        password: password,
    })
    .await
    .map_err(|e| AppError::Db { source: e })
    .map(|_| ())
}
