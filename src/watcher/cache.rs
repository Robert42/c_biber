use super::*;

pub struct Cache
{
  files: HashMap<Arc<Path>, blake3::Hash>,
  sender: mpsc::Sender<Event>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event
{
  ADD(Arc<Path>, Vec<u8>),
  MODIFIED(Arc<Path>, Vec<u8>),
}

impl Cache
{
  pub fn new() -> (Self, mpsc::Receiver<Event>)
  {
    let (sender, receiver) = mpsc::channel();

    let cache = Cache{
      files: HashMap::with_capacity(4096),
      sender
    };

    (cache, receiver)
  }

  pub fn add<P: AsRef<Path>, B: Into<Vec<u8>>>(&mut self, path: P, content: B) -> Result
  {
    let content = content.into();
    let new_hash = blake3::hash(content.as_slice());

    let path = path.as_ref();
    let path : Arc<Path> = Arc::from(path);
    if let Some(old_hash) = self.files.get(&path).copied()
    {
      if old_hash != new_hash
      {
        let _ = self.sender.send(Event::MODIFIED(path, content));
      }
    }
    else
    {
      self.files.insert(path.clone(), new_hash);
      let _ = self.sender.send(Event::ADD(path, content));
    }
    Ok(())
  }

  pub fn iter(&self) -> impl Iterator<Item=&Arc<Path>>
  {
    self.files.keys()
  }
}

#[cfg(test)]
mod test;