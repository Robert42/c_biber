use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Compiler
{
  GCC,
}

use Compiler::*;

impl Compiler
{
  pub fn path(self) -> &'static Path
  {
    match self
    {
      GCC => Path::new("gcc"),
    }
  }
}

pub fn find_compiler() -> Result<Vec<Compiler>>
{
  use Compiler::*;

  let mut compilers = vec![];
  for (compiler, arg) in [
    (GCC, "--version"),
  ]
  {
    if let Ok(output) = std::process::Command::new(compiler.path()).arg(arg).output()
    {
      if output.status.success()
      {
        compilers.push(compiler);
      }
    }
  }
  Ok(compilers)
}
