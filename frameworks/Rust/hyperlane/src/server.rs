use crate::*;

#[inline]
pub async fn run_server() {
    let mut server: Server = Server::new();
    server.host("0.0.0.0").await;
    server.port(8080).await;
    server.log_dir("./logs").await;
    server.log_interval_millis(1_000_000_000).await;
    server.disable_inner_log().await;
    server.disable_inner_print().await;
    server.route("/plaintext", plaintext).await;
    server.route("/json", json).await;
    server.route("/cached-quer", cached_queries).await;
    server.route("/db", db).await;
    server.route("/query", queries).await;
    server.route("/fortunes", fortunes).await;
    server.route("/upda", updates).await;
    server.request_middleware(request).await;
    server.response_middleware(response).await;
    server.listen().await;
}
