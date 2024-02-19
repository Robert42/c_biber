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

  pub fn compile<P: AsRef<Path>>(&self, file: P) -> Result<Compilation>
  {
    let child_process = self.cmd().arg(file.as_ref()).spawn()?;
    Ok(Compilation(child_process))
  }
}

pub struct Compilation(process::Child);

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
    .. GCC
  };
  pub const ZIG_CC : Compiler = Compiler
  {
    cmd: &["zig", "cc"],
    .. CLANG
  };
}

pub fn find_c_compiler() -> Result<Compiler>
{
  return find_compiler(cc::ALL.iter().copied());
}

pub fn find_compiler<Cs: IntoIterator<Item=Compiler>>(candidates: Cs) -> Result<Compiler>
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

  let mut result = Err(crate::Error::NO_COMPILER_FOUND);
  for (compiler, mut child) in subprocesses.into_iter().rev()
  {
    if let Ok(status) = child.wait()
    {
      if status.success()
      {
        result = Ok(compiler);
      }
    }
  }
  return result;
}

impl Compilation
{
  pub fn join(mut self) -> Result
  {
    let status = self.0.wait()?;
    if !status.success() { Err(Error::COMPILE_ERROR)?; } 
    Ok(())
  }
}

#[derive(Debug, Error)]
pub enum Error
{
  #[error("compile error")]
  COMPILE_ERROR
}