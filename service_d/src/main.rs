use axum::{Router, routing::get};

async fn get_data() -> String {
    "Data from Service D".to_string()
}

#[tokio::main]
async fn main() {
    // build our application with routes
    let app = Router::new()
        .route("/", get(|| async { "Hello from Service D!" }))
        .route("/data", get(get_data));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
