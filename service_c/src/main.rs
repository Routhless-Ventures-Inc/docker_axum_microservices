use axum::{Router, routing::get};

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

async fn get_data() -> String {
    let d_response = call_service_d().await;
    format!("Data from Service C (got from D: {})", d_response)
}

#[tokio::main]
async fn main() {
    // build our application with routes
    let app = Router::new()
        .route("/", get(|| async { "Hello from Service C!" }))
        .route("/data", get(get_data));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
