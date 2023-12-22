use super::*;

pub struct Cache
{
  files: Vec<Arc<Path>>,
  sender: mpsc::Sender<Event>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event
{
  ADD(Arc<Path>, Arc<[u8]>),
}

impl Cache
{
  pub fn new() -> (Self, mpsc::Receiver<Event>)
  {
    let (sender, receiver) = mpsc::channel();

    let cache = Cache{
      files: vec![],
      sender
    };

    (cache, receiver)
  }

  pub fn add<P: AsRef<Path>, B: AsRef<[u8]>>(&mut self, path: P, content: B) -> Result
  {
    let path = path.as_ref();
    let content = Arc::from(content.as_ref());
    let path : Arc<Path> = Arc::from(path);
    if self.files.contains(&path) {return Ok(())}
    self.files.push(path.clone());
    let _ = self.sender.send(Event::ADD(path, content));
    Ok(())
  }

  pub fn iter(&self) -> impl Iterator<Item=&Arc<Path>>
  {
    self.files.iter()
  }
}

#[cfg(test)]
mod test;