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

#[tokio::main]
async fn main() {
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
        .route("/users", post(post_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
