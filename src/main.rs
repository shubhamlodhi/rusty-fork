use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    
    let app = Router::new().route("/",get(||async {
        Html("
            <h1>Hello World</h1>
            <h2>Here Rust Server is running....</h2>
            Author: <strong>Yellow Coder</strong>
            ")
    
    }));
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener,app).await.unwrap();
}

// async fn handler()-> &'static str{
//     "Hello, world!"
// } 