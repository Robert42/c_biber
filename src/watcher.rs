use super::*;

pub mod cache;
pub mod scan;

pub use cache::Cache;

pub struct Watcher<F>
{
  filter: F,
  path: PathBuf,
  cache: Cache,

  fs_notify: notify::RecommendedWatcher,
  fs_notify_receiver: mpsc::Receiver<notify::Result<notify::Event>>,
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
  pub fn new<P: AsRef<Path>>(path: P, file_filter: F) -> Result<(Self, mpsc::Receiver<cache::Event>)>
  {
    let (fs_notify_sender, fs_notify_receiver) = mpsc::channel();

    let (cache, receiver) = Cache::new();
    let w = Watcher{
      filter: file_filter,
      path: path.as_ref().to_owned(),
      cache,
      fs_notify: notify::RecommendedWatcher::new(fs_notify_sender, notify::Config::default())?,
      fs_notify_receiver,
    };
    Ok((w, receiver))
  }
}

#[cfg(test)]
mod test;

use crate::notify::Watcher as Notify_Watcher;