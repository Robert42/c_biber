use super::*;

pub mod cache;
pub mod scan;

pub use cache::Cache;

pub enum Watch_Event
{
  CACHE_UPDATED(cache::Event),
  FAILURE(crate::Error),
  FIRST_SCAN_FINISHED,
}

pub fn watch<P, F>(path: P, file_filter: F) -> Result<mpsc::Receiver<Watch_Event>>
where
  P: AsRef<Path>,
  F: Fn(&Path)->Option<bool> + 'static + std::marker::Send,
{
  let (mut watch_sender, watch_receiver) = mpsc::channel();

  let path : PathBuf = path.as_ref().to_owned();

  std::thread::spawn(move ||
    match _watch(path, &mut watch_sender, file_filter)
    {
      Ok(_) => (),
      Err(e) => { let _ = watch_sender.send(Watch_Event::FAILURE(e)); },
    }
  );
  Ok(watch_receiver)
}

fn _watch<F>(path: PathBuf, watch_sender: &mut mpsc::Sender<Watch_Event>, file_filter: F) -> Result
  where F: Fn(&Path)->Option<bool>,
{
  let (mut cache, cache_update_receiver) = Cache::new();
  let (fs_notify_sender, fs_notify_receiver) = mpsc::channel();

  let mut fs_notify = notify::RecommendedWatcher::new(fs_notify_sender, notify::Config::default())?;

  fs_notify.watch(&path, notify::RecursiveMode::Recursive)?;
  cache.scan_files(&path, file_filter)?;

  while let Ok(e) = cache_update_receiver.try_recv()
  {
    watch_sender.send(Watch_Event::CACHE_UPDATED(e))?;
  }

  // Always send signal that the first scan succeeded
  watch_sender.send(Watch_Event::FIRST_SCAN_FINISHED)?;

  loop
  {
    let _ = fs_notify_receiver.recv()?;
  }
}

#[cfg(test)]
mod test;

use crate::notify::Watcher as Notify_Watcher;