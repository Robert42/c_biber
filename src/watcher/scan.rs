use super::*;

impl<Sender: cache::Event_Sender> Cache<Sender>
{
  pub fn scan_files<P: AsRef<Path>, F: Fn(&Path)->Option<bool>>(&mut self, path: P, filter: F) -> Result
  {
    let path = path.as_ref();
    for entry in WalkDir::new(path)
    {
      let path = entry?.into_path();
      if !path.is_file() {continue}
      if !(filter)(path.as_path()).unwrap_or(false) {continue}

      let content = fs::read(&path)?;
      let path = path.to_owned();

      self.add(path, content);
    }
  
    Ok(())
  }
}

use crate::walkdir::WalkDir;