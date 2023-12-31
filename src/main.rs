use axum::{routing::get, Router, response::IntoResponse, extract::Path, http::Request, body::Body};
use axum_server_backend::responders;

async fn root(Path(path): Path<String>, req: Request<Body>) -> impl IntoResponse {
    responders::file::open(format!("./sites/{}", path).into(), &req).await
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/*path", get(root));

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
