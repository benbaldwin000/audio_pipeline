use crate::fs_provider::FsAudioProvider;
use axum::{
    extract::{Query, State, BodyStream},
    routing::{get, put},
    Json, Router, Server,
};
use hyper::StatusCode;
use symphonia::core::io::ReadOnlySource;
use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

struct HttpGateway {
    provider: Arc<FsAudioProvider>,
}

struct GatewayHandlerState {
    provider: Arc<FsAudioProvider>,
}

type SharedGatewayHandlerState = State<Arc<GatewayHandlerState>>;

impl HttpGateway {
    pub fn new(provider: FsAudioProvider) -> Self {
        Self {
            provider: Arc::new(provider),
        }
    }

    pub async fn serve(&self, port: u16) {
        let address = SocketAddr::new(IpAddr::from([127, 0, 0, 1]), port);
        let handler_ctx = Arc::new(GatewayHandlerState {
            provider: self.provider.clone(),
        });
        let service = Router::new()
            .route("/audio", get(http_get_audio))
            .route("/audio", put(http_get_track_stream))
            .with_state(handler_ctx)
            .into_make_service();

        println!("starting server at: {:?}", address);
        Server::bind(&address).serve(service).await.unwrap();
    }
}

async fn http_upload_audio(
    State(state): SharedGatewayHandlerState,
    Query(params): Query<HashMap<String, String>>,
    mut stream: BodyStream
) -> Result<(), StatusCode> {
    let id = match params.get("id") {
        Some(id) => id,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    

    Ok(())
}

async fn http_get_audio(
    State(state): SharedGatewayHandlerState,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Vec<u8>, StatusCode> {
    let id = match params.get("id") {
        Some(id) => id,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    todo!()
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
    Query(params): Query<HashMap<String, String>>,
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
