#![allow(non_camel_case_types)]

extern crate walkdir;
#[macro_use] extern crate thiserror;

pub fn all_files_in_dir<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<std::path::PathBuf>, Error>
{
  use crate::walkdir::WalkDir;
  
  let mut paths = vec![];
  for entry in WalkDir::new(path)
  {
    let entry = entry?;
    paths.push(entry.into_path());
  }

  Ok(paths)
}

pub type Result<T=(), E=Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error
{
  #[error("io-error: {0}")]
  IO(#[from] std::io::Error),
  #[error("io-error: {0}")]
  FILE_WALKER_ERROR(#[from] walkdir::Error),
}