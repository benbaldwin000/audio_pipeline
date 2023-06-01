use crate::{pipeline::AudioPipeline, storage::Track};
use axum::{
    extract::{Query, State},
    routing::get,
    Router, Server, Json,
};
use hyper::StatusCode;
use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

struct GatewayHandlerState {
    pipeline: AudioPipeline,
}

type SharedGatewayHandlerState = State<Arc<GatewayHandlerState>>;

pub async fn start_http_audio_gateway(port: u16, pipeline: AudioPipeline) {
    let address = SocketAddr::new(IpAddr::from([127, 0, 0, 1]), port);
    let handler_ctx = Arc::new(GatewayHandlerState { pipeline });
    let service = Router::new()
        .route("/track", get(http_get_track))
        .route("/track/stream", get(http_get_track_stream))
        .with_state(handler_ctx)
        .into_make_service();

    println!("starting server at: {:?}", address);
    Server::bind(&address).serve(service).await.unwrap();
}

async fn http_get_track(
    State(state): SharedGatewayHandlerState,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Track>, StatusCode> {
    let id_str = match params.get("id") {
        Some(id) => id,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let id = match u64::from_str_radix(id_str, 16) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    return match state.pipeline.library.read().unwrap().get_track(id) {
        Ok(track) => Ok(Json((*track).clone())),
        Err(_) => Err(StatusCode::NOT_FOUND),
    };
}

async fn http_get_track_stream(
    State(state): SharedGatewayHandlerState,
    Query(params): Query<HashMap<String, String>>
) -> Result<Vec<u8>, StatusCode> {
    let id_str = match params.get("id") {
        Some(id) => id,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let id = match u64::from_str_radix(id_str, 16) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    return match state.pipeline.library.read().unwrap().get_track_source(id) {
        Ok(stream) => Ok(stream),
        Err(_) => Err(StatusCode::NOT_FOUND),
    };
}
