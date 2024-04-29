use std::sync::{ Arc };

use axum::{
    extract::{Request, State}, 
    response::{Response, IntoResponse, Json},
    http::{Method, HeaderMap},
    routing::{get, MethodRouter},
    Router,
};
use serde_json::{Value, json};

use crate::core::state::CoreStateDisk;
use crate::interface::chain::Engine;

/********************/

pub type ChainEngine = Arc<dyn Engine>;

#[derive(Clone)]
pub struct ApiCtx {
    pub engine: ChainEngine,
}


include!("util.rs");
include!("routes.rs");
include!("console.rs");
include!("balance.rs");



