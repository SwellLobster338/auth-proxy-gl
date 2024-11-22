use crate::injector::types::response;
use crate::state;
use axum::extract::{Path, State};
use axum::routing::{on, MethodFilter};
use axum::Json;

pub fn routes() -> axum::Router<state::State> {
    axum::Router::new().route("/", on(MethodFilter::GET, root))
}

async fn root(
    State(state): State<state::State>,
    Path(server_id): Path<String>,
) -> Json<response::root::Root> {
    let response = response::root::Root {
        meta: response::root::meta::Meta {
            server_name: Some(server_id),
            implementation_name: None,
            implementation_version: None,
        },
        skin_domains: Vec::new(),
        signature_public_key: state.config.keys.public.to_string(),
    };

    Json(response)
}
