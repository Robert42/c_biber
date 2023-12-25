use super::*;

pub struct Cache<Sender: Event_Sender>
{
  files: HashMap<Arc<Path>, blake3::Hash>,
  added: HashSet<Arc<Path>>,
  sender: Sender,
}

pub trait Event_Sender
{
  fn send(&self, event: Event);
}

impl Event_Sender for mpsc::Sender<Event>
{
  fn send(&self, event: Event)
  {
    let _ = mpsc::Sender::<Event>::send(self, event);
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event
{
  ADD(Arc<Path>, Vec<u8>),
  MODIFIED(Arc<Path>, Vec<u8>),
  REMOVE(Arc<Path>),
}

impl Cache<mpsc::Sender<Event>>
{
  pub fn new() -> (Self, mpsc::Receiver<Event>)
  {
    let (sender, receiver) = mpsc::channel();
    (Self::with_sender(sender), receiver)
  }
}

impl<Sender: Event_Sender> Cache<Sender>
{
  pub fn with_sender(sender: Sender) -> Self
  {
    Cache{
      files: HashMap::with_capacity(4096),
      added: HashSet::with_capacity(4096),
      sender
    }
  }

  pub fn add<P: AsRef<Path>, B: Into<Vec<u8>>>(&mut self, path: P, content: B)
  {
    let content = content.into();
    let new_hash = blake3::hash(content.as_slice());

    let path = path.as_ref();
    let path : Arc<Path> = Arc::from(path);
    self.added.insert(path.clone());
    if let Some(old_hash) = self.files.get(&path).copied()
    {
      if old_hash != new_hash
      {
        self.files.insert(path.clone(), new_hash);
        self.sender.send(Event::MODIFIED(path, content));
      }
    }
    else
    {
      self.files.insert(path.clone(), new_hash);
      self.sender.send(Event::ADD(path, content));
    }
  }

  pub fn remove<P: AsRef<Path>>(&mut self, path: P)
  {
    let path = path.as_ref();
    if let Some((path, _)) = self.files.remove_entry(path)
    {
      self.sender.send(Event::REMOVE(path));
    }
  }

  pub fn full_scan<F>(&mut self, scan: F) -> Result
  where F: Fn(&mut Self) -> Result
  {
    self.added.clear();
    scan(self)?;

    self.files.retain(|path,_|
      if self.added.contains(path) {true}
      else {self.sender.send(Event::REMOVE(path.clone())); false}
    );

    Ok(())
  }
}

#[cfg(test)]
mod test;