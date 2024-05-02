use std::sync::{ Arc };

use axum::{
    extract::{Query, Request, State}, 
    response::{Response, IntoResponse, Json},
    http::{header, Method, HeaderMap},
    routing::{get, MethodRouter},
    Router,
};
use serde_json::{Value, json};

use crate::sys::*;
use crate::core::field::*;
use crate::core::component::*;
use crate::core::state::{ CoreStateDisk, CoreStoreDisk };
use crate::protocol::block::*;

use crate::interface::field::*;
use crate::interface::protocol::*;
use crate::interface::chain::Engine;

/********************/

pub type ChainEngine = Arc<dyn Engine>;

#[derive(Clone)]
pub struct ApiCtx {
    pub engine: ChainEngine,
}


include!("util.rs");
include!("param.rs");
include!("routes.rs");
include!("console.rs");
include!("balance.rs");



