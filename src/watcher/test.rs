use super::*;

#[test]
fn find_all_c_files()
{
  assert_eq!(create_and_find_files([]).unwrap(), vec![] as Vec<&str>);
  assert_eq!(create_and_find_files(["a.c"]).unwrap(), vec!["a.c"]);
  assert_eq!(create_and_find_files(["a.c", "b.c", "c.rs"]).unwrap(), vec!["a.c", "b.c"]);
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
  let watcher = watch(root, is_c_file)?;
  for event in watcher.only_first_scan()
  {
    let event = event?;
    use cache::Event::*;
    let path = match event
    {
      MODIFIED(path, _) | ADD(path, _) => path,
      REMOVE(_) => unimplemented!(),
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