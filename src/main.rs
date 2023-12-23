extern crate c_biber;

fn main() -> c_biber::Result
{
  let curr_dir = std::env::current_dir()?;
  let (mut watcher, receiver) = c_biber::Watcher::new(curr_dir, |p| Some(p.extension()?=="rs"))?;
  watcher.scan()?;

  loop
  {
    let event = receiver.recv()?;

    use c_biber::watcher::cache::Event::*;
    let label = match &event
    {
      MODIFIED(..) => "MODIFIED",
      ADD(..) => "ADD",
      REMOVE(..) => "REMOVE",
    };
    match event
    {
      MODIFIED(path, content) | ADD(path, content) =>
        println!("== {label} {} ==\n```\n{}\n```\n",
          path.display(),
          std::str::from_utf8(content.as_slice()).unwrap().lines().take(3).collect::<Vec<_>>().join("\n")
        ),
      REMOVE(path) => println!("== {label} {} ==", path.display()),
    }
  }
}
