use anyhow::Context;
use axum::{extract::State, routing::get, Json, Router};
use rand::{self, Rng};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddr},
};
use tokio::net::TcpListener;

async fn root(_state: State<Arc<Mutex<HashMap<String, String>>>>) -> String {
    String::from("works\n")
}

async fn list(
    State(state): State<Arc<Mutex<HashMap<String, String>>>>,
) -> Json<HashMap<String, String>> {
    let kv = state.lock().unwrap();
    Json(kv.clone())
}

pub async fn serve() -> anyhow::Result<()> {
    let kv_svc = crate::service::Service::new();

    thread::spawn(move || {
        let mut i = 0;
        while i < 10 {
            thread::sleep(Duration::from_millis(50));

            let kv = kv_svc.clone();
            kv.insert("test".to_string(), "test".to_string());
            i += 1;
        }
        println!("done with svc struct");
    });

    let kv = HashMap::<String, String>::new();
    let api_context = Arc::new(Mutex::new(kv));

    let kv_add = Arc::clone(&api_context);
    thread::spawn(move || {
        let mut i = 0;
        loop {
            thread::sleep(Duration::from_millis(50));
            let mut kv_add_locked = kv_add.lock().unwrap();
            (*kv_add_locked).insert(i.to_string(), String::from("User ") + &i.to_string());
            i += 1;
        }
    });

    let kv_delete = Arc::clone(&api_context);
    thread::spawn(move || {
        let mut rand_generator = rand::thread_rng();
        loop {
            thread::sleep(Duration::from_millis(50));
            let mut kv_delete_locked = kv_delete.lock().unwrap();

            let keys: Vec<String> = kv_delete_locked.keys().cloned().collect();
            if keys.len() > 10 {
                let key_index = rand_generator.gen_range(0..keys.len());
                let td = &keys[key_index];

                (*kv_delete_locked).remove(&td.to_string());
            }
        }
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/list", get(list))
        .with_state(Arc::clone(&api_context));

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app)
        .await
        .context("error running HTTP server")
}
