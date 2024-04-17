use std::thread;
use std::time::Duration;
use std::sync::{Arc, Weak};

use chrono::Local;

use crate::sys::*;
use crate::config::NodeConf;

use crate::interface::chain::*;


include!("node.rs");
include!("start.rs");


