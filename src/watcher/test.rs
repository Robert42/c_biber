use super::*;

#[test]
fn find_all_c_files()
{
  assert_eq!(create_and_find_files([]).unwrap(), vec![] as Vec<&str>);
  assert_eq!(create_and_find_files(["a.c"]).unwrap(), vec!["a.c"]);
  assert_eq!(create_and_find_files(["a.c", "b.c", "c.rs"]).unwrap(), vec!["a.c", "b.c"]);
}

#[test]
fn handle_notifications() -> Result
{
  let temp_dir = create_files([("original_unmodified", b"same content")])?;
  let root = temp_dir.path();

  let mut files : Vec<(&'static str, &'static [u8])> = vec![];
  for event in watch(root, any_file)?.only_first_scan()
  {
    let event = event?;
    use cache::Event::*;
    let (path, content) = match event
    {
      MODIFIED(path, content) | ADD(path, content) => (path, content),
      REMOVE(_) => unreachable!(),
    };
    let path = diff_paths(path, root).unwrap();
    let path = format!("{}", path.display());
    files.push((path.leak(), content.leak()));
  }

  assert_eq!(files, vec![("original_unmodified", b"same content".as_slice())]);

  Ok(())
}

fn create_files<'a, Files, Content>(files: Files) -> Result<TempDir>
where
  Files: IntoIterator<Item=(&'a str, Content)>,
  Content: AsRef<[u8]>,
{
  let tmp_dir = TempDir::new("find_all_c_files").unwrap();

  let root = tmp_dir.path();
  for (rel_file, content) in files
  {
    let file = root.join(rel_file);
    fs::write(file, content)?;
  }

  Ok(tmp_dir)
}

fn collect_cache<'a, Files>(files: Files) -> Result<Vec<String>>
where
  Files: IntoIterator<Item=(&'a str, &'a [u8])>
{
  let tmp_dir = create_files(files)?;
  let root = tmp_dir.path();

  let mut files = vec![];
  for event in watch(root, is_c_file)?.only_first_scan()
  {
    let event = event?;
    use cache::Event::*;
    let path = match event
    {
      MODIFIED(path, _) | ADD(path, _) => path,
      REMOVE(_) => unreachable!(),
    };
    files.push(path);
  }

  Ok(files.iter().map(|x| format!("{}", diff_paths(x, root).unwrap().display())).collect())
}

fn create_and_find_files<'a, Files>(files: Files) -> Result<Vec<String>>
where
  Files: IntoIterator<Item=&'a str>
{
  let nothing = "".as_bytes();
  let mut files = collect_cache(files.into_iter().map(|f| (f, nothing)))?;
  files.sort();
  Ok(files)
}

fn is_c_file(path: &Path) -> Option<bool>
{
  Some(path.extension()?=="c")
}

fn any_file(_: &Path) -> Option<bool>
{
  Some(true)
}