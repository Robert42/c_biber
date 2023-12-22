use super::*;

pub struct Cache
{
  files: Vec<Arc<Path>>,
  sender: mpsc::Sender<Event>,
}

pub type Event = ();

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

  pub fn add<P: AsRef<Path>>(&mut self, path: P) -> Result
  {
    self.files.push(Arc::from(path.as_ref()));
    let _ = self.sender.send(());
    Ok(())
  }

  pub fn iter(&self) -> impl Iterator<Item=&Arc<Path>>
  {
    self.files.iter()
  }
}

#[cfg(test)]
mod test;