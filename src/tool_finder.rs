use super::*;

#[derive(Clone, Debug)]
pub enum Compiler
{
  GCC(PathBuf),
}

pub fn find_compiler() -> Result<Option<Compiler>>
{
  let path = std::env::var("PATH")?;
  let sep = ':'; // TODO: Add Windows support
  let paths = path.split(sep);
  for read_dir in paths.filter_map(|path| fs::read_dir(path).ok())
  {
    for path in read_dir
      .filter_map(|e| e.ok())
      .filter_map(|e| if e.file_type().ok()?.is_file() {return Some(e)} else {Some(e)} )
      .filter_map(|e|
        {
          let t = e.file_type().ok()?;
          if t.is_file() || t.is_symlink() { return Some(e.path()) };
          None
        })
    {
      let file_name = if let Some(f) = path.file_name() {f} else {continue};
      if file_name == "gcc"
      {
        if let Ok(output) = std::process::Command::new("gcc").arg("--version").output()
        {
          if output.status.success()
          {
            return Ok(Some(Compiler::GCC(path)));
          }
        }
      }

    }
  }
  Ok(None)
}
