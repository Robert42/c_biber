use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Compiler
{
  GCC,
  CLANG,
}

use Compiler::*;

impl Compiler
{
  pub fn command(self) -> std::process::Command
  {
    match self
    {
      GCC => std::process::Command::new("gcc"),
      CLANG => std::process::Command::new("clang"),
    }
  }
}

pub fn find_compiler() -> Result<Vec<Compiler>>
{
  use Compiler::*;

  let mut compilers = vec![];
  for (compiler, arg) in [
    (GCC, "--version"),
    (CLANG, "--version"),
  ]
  {
    if let Ok(output) = compiler.command().arg(arg).output()
    {
      if output.status.success()
      {
        compilers.push(compiler);
      }
    }
  }
  Ok(compilers)
}
