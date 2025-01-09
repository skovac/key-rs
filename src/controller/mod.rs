use anyhow::Context;
use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr},
};
use tokio::net::TcpListener;

use crate::service;

#[derive(Clone)]
struct ApiContext {
    service: service::Service,
}

#[derive(Serialize)]
struct User {
    id: String,
    name: String,
}

async fn root(_state: State<ApiContext>) -> String {
    String::from("works\n")
}

async fn list(state: State<ApiContext>) -> Json<HashMap<String, String>> {
    let kv = state.service.get_all().lock().unwrap();
    Json(*kv)
}

pub async fn serve(service: service::Service) -> anyhow::Result<()> {
    let api_context = ApiContext { service: service };

    let app = Router::new()
        .route("/", get(root))
        .route("/list", get(list))
        .with_state(api_context);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .await
        .context("error running HTTP server")
}
