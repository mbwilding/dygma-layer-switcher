pub mod layer;
pub mod structs;
pub mod verbiage;

pub const MAX_LAYERS: u8 = 10;

use std::sync::{Arc, Mutex};
use structs::Configuration;

lazy_static::lazy_static! {
    pub static ref CONFIGURATION: Arc<Mutex<Configuration>> =
        Arc::new(Mutex::new(Configuration::default()));
}
