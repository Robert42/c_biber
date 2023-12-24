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

pub struct Watcher(mpsc::Receiver<Watch_Event>);

impl Watcher
{
  pub fn only_first_scan(self) -> impl Iterator<Item=Result<cache::Event>>
  {
    use Watch_Event::*;
    let i = self.0.into_iter()
      .take_while(|x| match x { FIRST_SCAN_FINISHED => false, _ => true} );
    convert_iterator(i)
  }

  pub fn watch(self) -> impl Iterator<Item=Result<cache::Event>>
  {
    convert_iterator(self.0.into_iter())
  }

  pub fn poll_timeout(&self, timeout: Duration) -> impl Iterator<Item = Result<cache::Event>> + '_
  {
    let i = self.0.recv_timeout(timeout).ok().into_iter().chain(self.0.try_iter());
    convert_iterator(i)
  }
}

pub fn watch<P, F>(path: P, file_filter: F) -> Result<Watcher>
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
  Ok(Watcher(watch_receiver))
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

fn convert_iterator<I: IntoIterator<Item=Watch_Event>>(i: I) -> impl Iterator<Item=Result<cache::Event>>
{
  use Watch_Event::*;
  i.into_iter().filter(|x| match x { FIRST_SCAN_FINISHED => false, _ => true} )
    .map(
    |x|
    match x
    {
      CACHE_UPDATED(update) => Ok(update),
      FAILURE(e) => Err(e),
      FIRST_SCAN_FINISHED => unreachable!(),
    }
  )
}