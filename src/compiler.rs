use super::*;

#[derive(Clone, Copy, Debug)]
pub enum Compiler
{
  GCC,
  CLANG,
  ZIG_CC,
}

use Compiler::*;

impl Compiler
{
  pub fn command(self) -> process::Command
  {
    match self
    {
      GCC => process::Command::new("gcc"),
      CLANG => process::Command::new("clang"),
      ZIG_CC => {let mut c = process::Command::new("zig"); c.arg("cc"); c}
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
    (ZIG_CC, "--version"),
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
