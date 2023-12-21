extern crate c_biber;

fn main() -> std::io::Result<()>
{
  let curr_dir = std::env::current_dir()?;
  dbg!(c_biber::all_files_in_dir(curr_dir));

  Ok(())
}
