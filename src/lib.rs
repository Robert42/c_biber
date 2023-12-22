#![allow(non_camel_case_types)]

pub mod watcher;
pub use watcher::Watcher;

extern crate walkdir;
#[macro_use] extern crate thiserror;

pub type Result<T=(), E=Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error
{
  #[error("io-error: {0}")]
  IO(#[from] std::io::Error),
  #[error("io-error: {0}")]
  FILE_WALKER_ERROR(#[from] walkdir::Error),
}