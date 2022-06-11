use salvo::prelude::*;
use salvo::extra::serve_static::FileHandler;

#[fn_handler]
async fn hello_world() -> &'static str {
    "hello_world"
}

#[tokio::main]
async fn main() {
    // let router = Router::new().get(hello_world);
    let router = Router::new().push(
        Router::new()
        .path("")
        .get(FileHandler::new("public/index.html"))
    );
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router).await
}
