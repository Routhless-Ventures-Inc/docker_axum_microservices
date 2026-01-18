use axum::{Router, extract::Query, routing::get};
use serde::Deserialize;

#[derive(Deserialize)]
struct GreetQuery {
    name: Option<String>,
}

async fn call_service_c() -> String {
    let client = reqwest::Client::new();
    match client.get("http://service_c:3000/data").send().await {
        Ok(response) => match response.text().await {
            Ok(text) => text,
            Err(_) => "Failed to read response from C".to_string(),
        },
        Err(_) => "Failed to call service_c".to_string(),
    }
}

async fn call_service_d() -> String {
    let client = reqwest::Client::new();
    match client.get("http://service_d:3000/data").send().await {
        Ok(response) => match response.text().await {
            Ok(text) => text,
            Err(_) => "Failed to read response from D".to_string(),
        },
        Err(_) => "Failed to call service_d".to_string(),
    }
}

async fn greet(Query(params): Query<GreetQuery>) -> String {
    let name = params.name.unwrap_or_else(|| "World".to_string());
    let c_response = call_service_c().await;
    let d_response = call_service_d().await;
    format!(
        "Hello, {}! This is Service B. Got from C: {}. Got from D: {}.",
        name, c_response, d_response
    )
}

#[tokio::main]
async fn main() {
    // build our application with routes
    let app = Router::new()
        .route("/", get(|| async { "Hello from Service B!" }))
        .route("/greet", get(greet));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
