use std::path::*;
use std::sync::{Arc, Mutex, mpsc};
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering::{self, Relaxed} };

use std::time::*;
use std::thread::*;


use crate::x16rs;
use crate::sys;
use crate::sys::*;
use crate::config;
use crate::config::*;
use crate::core::field::*;
use crate::base::field::*;
use crate::interface::extend::*;
use crate::interface::field::*;
use crate::interface::chain::*;
use crate::interface::protocol::*;
use crate::core::account::Account;
use crate::mint::checker::*;
use crate::mint::component::*;
use crate::mint::coinbase::*;
use crate::chain::engine::*;
use crate::node::node::*;
use crate::server::*;
use crate::mint::action::*;
use crate::mint::difficulty::*;
use crate::protocol::block::*;
use crate::protocol::transaction::*;



use reqwest::blocking::Client as HttpClient;
use serde_json::{ Value as JV };


const MINING_INTERVAL: f64 = 3.0; // 3 secs


include!("util.rs");
include!("fullnode.rs");  // Fullnode
include!("poworker.rs");  // HAC PoW worker
include!("diaworker.rs"); // Diamond worker



