use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Compiler
{
  pub cmd: &'static [&'static str],
  pub get_version: &'static str,
}

impl Compiler
{
  pub fn cmd(&self) -> process::Command
  {
    let mut cmd = process::Command::new(self.cmd[0]);
    for arg in &self.cmd[1..] {cmd.arg(arg);}
    cmd
  }
}

pub mod cc
{
  use super::*;

  pub const ALL : &'static [Compiler] = &[GCC, CLANG, ZIG_CC];

  pub const GCC : Compiler = Compiler
  {
    cmd: &["gcc"],
    get_version: "--version",
  };
  pub const CLANG : Compiler = Compiler
  {
    cmd: &["clang"],
    get_version: "--version",
  };
  pub const ZIG_CC : Compiler = Compiler
  {
    cmd: &["zig", "cc"],
    get_version: "--version",
  };
}

pub fn find_c_compiler() -> Result<Vec<Compiler>>
{
  return find_compiler(cc::ALL.iter().copied());
}

pub fn find_compiler<Cs: IntoIterator<Item=Compiler>>(candidates: Cs) -> Result<Vec<Compiler>>
{
  let mut subprocesses = vec![];
  for compiler in candidates
  {
    if let Ok(child) = compiler.cmd()
      .arg(compiler.get_version)
      .stdin(process::Stdio::null())
      .stdout(process::Stdio::null())
      .stderr(process::Stdio::null())
      .spawn()
    {
      subprocesses.push((compiler, child));
    }
  }

  let mut compilers = vec![];
  for (compiler, mut child) in subprocesses.into_iter()
  {
    if let Ok(status) = child.wait()
    {
      if status.success()
      {
        compilers.push(compiler);
      }
    }
  }
  Ok(compilers)
}
