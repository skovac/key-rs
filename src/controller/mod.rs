use crate::service::Service;
use anyhow::Context;
use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct KeyValue {
    value: String,
}

async fn root(_state: State<Service>) -> String {
    String::from("key-rs key-value store.\ngit@github.com:skovac/key-rs.git\n")
}

// GET /key- Get all keys and values
async fn all(State(service): State<Service>) -> Json<HashMap<String, String>> {
    Json(service.kv.all())
}

// GET /list - Get all keys and values
async fn list(State(service): State<Service>) -> Json<HashMap<String, String>> {
    Json(service.kv.all())
}

// GET /key/{key} - Get a value by key
async fn get_value(
    Path(key): Path<String>,
    State(service): State<Service>,
) -> Json<Option<String>> {
    Json(service.kv.get(&key))
}

// POST /key/{key} - Insert or update a value
async fn insert_value(
    Path(key): Path<String>,
    State(service): State<Service>,
    Json(payload): Json<KeyValue>,
) {
    service.kv.insert(key, payload.value);
}

// DELETE /key/{key} - Delete a value
async fn delete_value(
    Path(key): Path<String>,
    State(service): State<Service>,
) -> Json<Option<String>> {
    Json(service.kv.remove(&key))
}

pub async fn serve(service: Service) -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(root))
        .route("/key", get(all))
        .route("/key/{key}", get(get_value))
        .route("/key/{key}", post(insert_value))
        .route("/key/{key}", delete(delete_value))
        .with_state(service);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .await
        .context("error running HTTP server")
}
