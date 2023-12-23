use super::*;

pub mod cache;
pub mod scan;

pub use cache::Cache;

pub struct Watcher<F>
{
  filter: F,
  path: PathBuf,
  cache: Cache,
}

impl<F> Watcher<F>
{
  pub fn cache(&self) -> &Cache
  {
    &self.cache
  }
}

impl<F> Watcher<F>
  where
    F: Fn(&Path)->Option<bool>
{
  pub fn new<P: AsRef<Path>>(path: P, file_filter: F) -> Self
  {
    let (cache, _) = Cache::new();
    Watcher{
      filter: file_filter,
      path: path.as_ref().to_owned(),
      cache,
    }
  }
}

#[cfg(test)]
mod test;