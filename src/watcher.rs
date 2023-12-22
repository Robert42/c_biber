use super::*;

pub mod scan;

pub struct Watcher<F>
{
  filter: F,
  path: PathBuf,
  pub cache: Vec<PathBuf>
}

impl<F> Watcher<F>
  where
    F: Fn(&Path)->Option<bool>
{
  pub fn new<P: AsRef<Path>>(path: P, file_filter: F) -> Self
  {
    Watcher{
      filter: file_filter,
      path: path.as_ref().to_owned(),
      cache: vec![],
    }
  }
}
