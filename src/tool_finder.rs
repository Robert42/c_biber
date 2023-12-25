use super::*;

#[derive(Clone, Debug)]
pub enum Compiler
{
  GCC(PathBuf),
}

pub fn find_compiler() -> Result<Option<Compiler>>
{
  if let Ok(output) = std::process::Command::new("gcc").arg("--version").output()
  {
    if output.status.success()
    {
      return Ok(Some(Compiler::GCC(PathBuf::from("gcc"))));
    }
  }
  Ok(None)
}
