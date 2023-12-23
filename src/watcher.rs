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
  pub fn new<P: AsRef<Path>>(path: P, file_filter: F) -> (Self, mpsc::Receiver<cache::Event>)
  {
    let (cache, receiver) = Cache::new();
    let w = Watcher{
      filter: file_filter,
      path: path.as_ref().to_owned(),
      cache,
    };
    (w, receiver)
  }
}

#[cfg(test)]
mod test;