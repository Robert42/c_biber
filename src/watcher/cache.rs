use super::*;

pub struct Cache
{
  files: Vec<PathBuf>,
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

  pub fn add(&mut self, path: PathBuf) -> Result
  {
    self.files.push(path);
    let _ = self.sender.send(());
    Ok(())
  }

  pub fn iter(&self) -> impl Iterator<Item=&PathBuf>
  {
    self.files.iter()
  }
}

#[cfg(test)]
mod test;