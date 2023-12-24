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

  let watcher = watch(root, any_file)?;
  let updates = Updates::new(root, watcher.poll_timeout(Duration::from_millis(2)))?;

  assert_eq!(updates.added, vec![("original_unmodified", b"same content".as_slice())]);

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

fn collect_cache<'a, Files>(files: Files) -> Result<Vec<&'static str>>
where
  Files: IntoIterator<Item=(&'a str, &'a [u8])>
{
  let tmp_dir = create_files(files)?;
  let root = tmp_dir.path();

  let watcher = watch(root, is_c_file)?;
  let updates = Updates::new(root, watcher.only_first_scan())?;

  return Ok(updates.added.into_iter().map(|(p,_)| p).collect());
}

fn create_and_find_files<'a, Files>(files: Files) -> Result<Vec<&'static str>>
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

#[derive(Default)]
struct Updates
{
  added: Vec<(&'static str, &'static [u8])>,
  modified: Vec<(&'static str, &'static [u8])>,
  removed: Vec<&'static str>,
}

impl Updates
{
  fn new<T: IntoIterator<Item = Result<cache::Event>>, P: AsRef<Path>>(root: P, iter: T) -> Result<Self>
  {
    let root = root.as_ref();
    let leak_path = |path: Arc<Path>| -> &'static str
    {
      let path = diff_paths(path, root).unwrap();
      format!("{}", path.display()).leak()
    };

    let mut updates = Updates::default();
    for event in iter
    {
      let event = event?;
      use cache::Event::*;
      match event
      {
        ADD(path, content) => updates.added.push((leak_path(path), content.leak())),
        MODIFIED(path, content) => updates.modified.push((leak_path(path), content.leak())),
        REMOVE(path) => updates.removed.push(leak_path(path)),
      };
    }

    return Ok(updates);
  }
}