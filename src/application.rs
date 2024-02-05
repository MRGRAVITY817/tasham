use {
    crate::{
        pages::hello,
        result::AppResult,
        settings::{
            database_settings::{AppDb, DatabaseSettings},
            Settings,
        },
    },
    axum::{routing::get, Router},
    std::sync::Arc,
    surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal},
    tower_livereload::LiveReloadLayer,
};

pub struct AppState {
    pub db: AppDb,
}

pub async fn build_app(settings: Settings) -> AppResult<()> {
    let app_state = Arc::new(AppState {
        db: connect_db(&settings.database).await?,
    });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3333").await?;

    // TODO: Add services
    let router = Router::new()
        .route("/", get(hello::hello_page))
        // .nest("/signup", signup::signup_routes())
        // .fallback_service(
        //     ServeDir::new("./static").not_found_service(notfound::not_found.into_service()),
        // )
        .layer(LiveReloadLayer::new())
        .with_state(app_state);

    axum::serve(listener, router).await?;

    Ok(())
}

async fn connect_db(settings: &DatabaseSettings) -> AppResult<AppDb> {
    // Connect to the server
    let db = Surreal::new::<Ws>(&settings.url).await?;

    // Signin as a namespace, database, or root user
    db.signin(Root {
        username: &settings.username,
        password: &settings.password,
    })
    .await?;

    Ok(db)
}
