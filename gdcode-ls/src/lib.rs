use jsonrpsee_ws_client::{HeaderMap, HeaderValue, WsClient, WsClientBuilder};

#[tokio::main]
pub async fn build_client(host: &str, port: &i32) -> WsClient {
    let mut headers = HeaderMap::new();
    headers.insert("Any-Header-You-Like", HeaderValue::from_static("42"));

    WsClientBuilder::default()
        .set_headers(headers)
        .build(format!("ws://{host}:{port}").as_str())
        .await
        .unwrap()
}
