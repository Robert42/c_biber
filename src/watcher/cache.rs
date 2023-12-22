use super::*;

#[derive(Default)]
pub struct Cache
{
  files: Vec<PathBuf>,
}

impl Cache
{
  pub fn add(&mut self, path: PathBuf) -> Result
  {
    self.files.push(path);
    Ok(())
  }

  pub fn iter(&self) -> impl Iterator<Item=&PathBuf>
  {
    self.files.iter()
  }
}