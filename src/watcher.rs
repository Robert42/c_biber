use super::*;

pub mod scan;

pub struct Watcher<F>
{
  filter: F,
  path: std::path::PathBuf,
}

impl<F> Watcher<F>
  where
    F: Fn(&std::path::Path)->Option<bool>
{
  pub fn new<P: AsRef<std::path::Path>>(path: P, file_filter: F) -> Self
  {
    Watcher{
      filter: file_filter,
      path: path.as_ref().to_owned(),
    }
  }
}
