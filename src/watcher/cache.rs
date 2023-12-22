use super::*;

pub struct Cache
{
  files: HashMap<Arc<Path>, Vec<u8>>,
  sender: mpsc::Sender<Event>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event
{
  ADD(Arc<Path>, Vec<u8>),
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

  pub fn add<P: AsRef<Path>>(&mut self, path: P, content: Vec<u8>) -> Result
  {
    let path = path.as_ref();
    let path : Arc<Path> = Arc::from(path);
    if let Some(old_content) = self.files.get(&path)
    {
      if old_content != &content
      {
        let _ = self.sender.send(Event::ADD(path, content));
      }
    }
    else
    {
      self.files.insert(path.clone(), content.clone());
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