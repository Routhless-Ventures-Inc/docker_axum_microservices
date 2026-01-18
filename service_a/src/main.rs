use axum::{Router, routing::get};

async fn call_service_b() -> String {
    let client = reqwest::Client::new();
    match client
        .get("http://service_b:3000/greet?name=ServiceA")
        .send()
        .await
    {
        Ok(response) => match response.text().await {
            Ok(text) => text,
            Err(_) => "Failed to read response".to_string(),
        },
        Err(_) => "Failed to call service_b".to_string(),
    }
}

#[tokio::main]
async fn main() {
    // build our application with routes
    let app = Router::new()
        .route("/", get(|| async { "Hello from Service A!" }))
        .route("/call-service-b", get(|| async { call_service_b().await }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
