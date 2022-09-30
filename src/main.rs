use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Router
};
use std::{io, net::SocketAddr};
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {

    let app: _ = Router::new()
        .route("/foo", get(|| async {
            "Hi from /foo"
        }))
        .fallback(
            get_service(ServeDir::new("./static")).handle_error(handle_error)
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
