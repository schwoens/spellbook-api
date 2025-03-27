use axum::{
    middleware,
    routing::{get, patch, post},
    Router,
};
use spellbook_api::handlers::{
    spells::{
        copy_public_spell, delete_spell, get_spell, get_spells, post_spell, publish_spell,
        query_public_spells, query_spells, unpublish_spell, update_spell,
    },
    users::post_user,
};
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {

    let cors_layer = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    let app = Router::new()
        .route("/spells", get(get_spells).post(post_spell))
        .route("/spells/query", post(query_spells))
        .route(
            "/spell/:nanoid",
            get(get_spell).put(update_spell).delete(delete_spell),
        )
        .route("/spell/publish/:nanoid", patch(publish_spell))
        .route("/spell/unpublish", patch(unpublish_spell))
        .route("/public/spells/query", post(query_public_spells))
        .route("/public/spell/copy/:nanoid", patch(copy_public_spell))
        .layer(middleware::from_fn(spellbook_api::middleware::auth))
        .route("/users", post(post_user))
        .layer(cors_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on port 3000...");
    axum::serve(listener, app).await.unwrap();
}
