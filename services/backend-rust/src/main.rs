use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use mongodb::{Client, Collection, bson::doc};
use tracing_subscriber;
use chrono; // Add chrono import
use axum_prometheus::PrometheusMetricLayer; // Fix Prometheus imports

#[derive(Debug, Serialize, Deserialize)]
struct Vote {
    voter_id: String,
    choice: String,
    ts: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Block {
    index: i64,
    vote_hash: String,
    prev_hash: String,
    timestamp: i64,
    hash: String,
}

struct AppState {
    votes: Collection<mongodb::bson::Document>,
    blocks: Collection<mongodb::bson::Document>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    // MongoDB URI from env var
    let mongo_uri = std::env::var("MONGO_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());
    let client = Client::with_uri_str(&mongo_uri).await?;
    let db = client.database("voting");
    let votes = db.collection("votes");
    let blocks = db.collection("blocks");
    let state = Arc::new(AppState { votes, blocks });

    let app = Router::new()
        .route("/vote", post(handle_vote))
        .route("/results", get(handle_results));

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let app = app
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .with_state(state);

    // run
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

async fn handle_vote(State(state): State<Arc<AppState>>, Json(payload): Json<Vote>) -> Result<Json<serde_json::Value>, anyhow::Error> {
    // insert vote
    let vote_doc = mongodb::bson::to_document(&payload)?;
    state.votes.insert_one(vote_doc, None).await?;

    // create block
    let vote_json = serde_json::to_string(&payload)?;
    let vote_hash = hex::encode(Sha256::digest(vote_json));

    // fetch last block
    let last_block_doc = state.blocks
        .find_one(None, mongodb::options::FindOneOptions::builder().sort(doc!{"index": -1}).build())
        .await?;

    let (prev_hash, index) = if let Some(doc) = last_block_doc {
        let prev_hash = doc.get_str("hash")?.to_string();
        let index = doc.get_i64("index")? + 1;
        (prev_hash, index)
    } else {
        ("0".to_string(), 0) // Genesis block
    };

    let mut new_block = Block {
        index,
        vote_hash,
        prev_hash,
        timestamp: chrono::Utc::now().timestamp(),
        hash: String::new(), // Will be calculated
    };

    let serialized_for_hash = serde_json::to_string(&new_block)?;
    let block_hash = hex::encode(Sha256::digest(serialized_for_hash));
    new_block.hash = block_hash;

    let to_insert = mongodb::bson::to_document(&new_block)?;
    state.blocks.insert_one(to_insert, None).await?;

    Ok(Json(serde_json::json!({"ok": true})))
}

async fn handle_results(State(state): State<Arc<AppState>>) -> Result<Json<serde_json::Value>, anyhow::Error> {
    // naive aggregation
    let mut cursor = state.votes.aggregate(vec![
        doc! { "$group": { "_id": "$choice", "count": { "$sum": 1 } } }
    ], None).await?;
    let mut out = serde_json::Map::new();
    while let Some(doc) = cursor.try_next().await? {
        let k = doc.get_str("_id")?.to_string();
        let v = doc.get_i32("count")?;
        out.insert(k, serde_json::Value::from(v));
    }
    Ok(Json(serde_json::Value::Object(out)))
}
