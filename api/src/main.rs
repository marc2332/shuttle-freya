use std::collections::HashMap;

use common::Message;

use axum::{
    routing::{get, post},
    response::Json,
    Router, extract::{Query, State},
};
use serde::{Deserialize, Serialize};
use shuttle_persist::PersistInstance;

async fn messages(State(state): State<ApiState>) -> Json<Vec<Message>> {
    let messages = state.persist.load::<Vec<Message>>("messages").unwrap_or_default();

    Json(messages)
}


async fn send(State(state): State<ApiState>, Query(params): Query<HashMap<String, String>>) {
    let text = params.get("text");
    let author = params.get("author");
    
    if let Some((text, author)) = text.zip(author) {
        let mut messages = state.persist.load::<Vec<Message>>("messages").unwrap_or_default();

        messages.push(Message {
            text: text.clone(),
            author: author.clone()
        });

        state.persist.save("messages", messages).ok();

    }
}

#[derive(Clone, Deserialize, Serialize)]
struct ApiState {
    persist: PersistInstance
}

#[shuttle_runtime::main]
async fn axum(#[shuttle_persist::Persist] persist: PersistInstance) -> shuttle_axum::ShuttleAxum {
    let state = ApiState {
        persist
    };

    let router = Router::new()
        .route("/messages", get(messages))
        .route("/send", post(send))
        .with_state(state);

    Ok(router.into())
}
