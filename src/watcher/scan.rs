use super::*;

impl<F> Watcher<F>
where
  F: Fn(&Path)->Option<bool>
{
  pub fn scan(&mut self) -> Result
  {
    for entry in WalkDir::new(&self.path)
    {
      let path = entry?.into_path();
      if !path.is_file() {continue}
      if !(self.filter)(path.as_path()).unwrap_or(false) {continue}

      let path = path.to_owned();

      self.cache.push(path);
    }
  
    Ok(())
  }
}

use crate::walkdir::WalkDir;