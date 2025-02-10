use std::env;

use axum::{
    routing::{get, post},
    Router,
};
use spellbook_api::handlers::{
    spells::{
        delete_spell, get_spell, get_spells, post_spell, publish_spell, query_public_spells,
        unpublish_spell, update_spell,
    },
    users::post_user,
};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or("3000".to_string());

    let app = Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/spells", get(get_spells).post(post_spell))
        .route(
            "/spell",
            get(get_spell).put(update_spell).delete(delete_spell),
        )
        .route("/spell/publish", post(publish_spell))
        .route("/spell/unpublish", post(unpublish_spell))
        .route("/public/spell/query", post(query_public_spells))
        .route("/users", post(post_user))
        .layer(CorsLayer::new().allow_origin(Any));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("listening on port {}...", port);
    axum::serve(listener, app).await.unwrap();
}
