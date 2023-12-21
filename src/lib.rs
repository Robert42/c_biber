extern crate walkdir;

pub fn all_files_in_dir<P: AsRef<std::path::Path>>(path: P) -> Vec<std::path::PathBuf>
{
  use crate::walkdir::WalkDir;
  
  let mut paths = vec![];
  for entry in WalkDir::new(path)
  {
    let entry = entry.unwrap();
    paths.push(entry.into_path());
  }

  paths
}