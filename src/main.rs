use salvo::prelude::*;

#[fn_handler]
async fn hello_world() -> &'static str {
    "hello_world"
}

#[tokio::main]
async fn main() {
    let router = Router::new().get(hello_world);
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .serve(router).await
}
