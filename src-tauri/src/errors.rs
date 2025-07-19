use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BazecorError {
    #[error("Not connected")]
    NotConnectedError,
}
