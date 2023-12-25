#![allow(non_camel_case_types)]

pub mod watcher;
pub use watcher::{watch, Watch_Event};

mod dev_prelude;
use dev_prelude::*;

extern crate notify;
#[cfg(test)]
extern crate pathdiff;
extern crate walkdir;
#[macro_use] extern crate thiserror;

#[cfg(test)] extern crate tempdir;

pub type Result<T=(), E=Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error
{
  #[error("io-error: {0}")]
  IO(#[from] std::io::Error),
  #[error("io-error: {0}")]
  FILE_WALKER_ERROR(#[from] walkdir::Error),
  #[error("channel-error (receiver): {0}")]
  CHANNEL_ERROR(#[from] mpsc::RecvError),
  #[error("channel-error (sender)")]
  CHANNEL_SEND_ERROR,
  #[error("notify-error: {0}")]
  NOTIFY_ERROR(#[from] notify::Error),
}

impl From<mpsc::SendError<Watch_Event>> for Error
{
  fn from(_value: mpsc::SendError<Watch_Event>) -> Self
  {
    Error::CHANNEL_SEND_ERROR
  }
}