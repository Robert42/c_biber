extern crate c_biber;

fn main() -> c_biber::Result
{
  let curr_dir = std::env::current_dir()?;
  let mut watcher = c_biber::Watcher::new(curr_dir, |p| Some(p.extension()?=="rs"));
  watcher.scan()?;

  Ok(())
}
