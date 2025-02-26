mod config;
mod controller;
mod provider;
mod service;

#[tokio::main]
async fn main() {
    let _config = config::Config {};
    let _provider = provider::Provider {};
    let service = service::Service::new();
    let _ = controller::serve(service).await;
}
