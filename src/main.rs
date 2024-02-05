mod application;
mod pages;
mod result;
mod settings;

use {crate::settings::Settings, application::build_app, result::AppResult};

#[tokio::main]
async fn main() -> AppResult<()> {
    let settings = Settings::new()?;

    build_app(settings).await
}
