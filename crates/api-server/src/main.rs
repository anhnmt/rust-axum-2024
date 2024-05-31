use tokio::net::TcpListener;
use api_server::api::router::create_router;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = create_router();

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("🚀 Server started successfully");
    axum::serve(listener, app).await.unwrap();
}