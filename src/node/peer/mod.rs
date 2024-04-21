use std::time::SystemTime;
use std::net::SocketAddr;
use std::sync::atomic::{ AtomicU64, Ordering };
use std::sync::{ Arc, Mutex as StdMutex };

use tokio::net::*;
use tokio::net::tcp::*;

use crate::sys::*;
use crate::base::field::*;
use crate::interface::field::*;

use super::p2p::*;



pub const PEER_ID_SIZE: usize = 16;
pub type PeerID = [u8; PEER_ID_SIZE];



include!("peer.rs");

