mod git_scraper;

use axum::{
    extract::Extension,
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tera::{Context, Tera};
use tower_http::services::{ServeDir, ServeFile};

async fn index(Extension(tera): Extension<Tera>) -> Html<String> {
    let gw = git_scraper::GitScraper::new().await;
    let mut context = Context::new();
    context.insert("title", "Sten Heimbrodt");
    context.insert("message", "just wannted to learn tera lol");
    context.insert("repo_number", &gw.repo_number.clone());

    let rendered = tera
        .render("index.html", &context)
        .unwrap_or_else(|err| format!("Error rendering template: {}", err));

    Html(rendered)
}

#[tokio::main]
async fn main() {
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Error parsing templates: {}", e);
            std::process::exit(1);
        }
    };
    let serve_dir = ServeDir::new("static").not_found_service(ServeFile::new("templates/index.html"));
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/static", serve_dir.clone())
        .layer(Extension(tera));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Server started on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}