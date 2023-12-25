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
  let temp_dir = create_files([
    ("original_unmodified.c", b"same content".as_slice()),
    ("to be modified.c", b"original content".as_slice()),
    ("to be removed.c", b"content to be removed".as_slice()),
  ])?;
  let root = temp_dir.path();

  let watcher = watch(root, is_c_file)?;
  
  // init
  let updates = Updates::new(root, watcher.poll_timeout(Duration::from_millis(2)))?;
  assert_eq!(updates, Updates{
    added: vec![
      ("original_unmodified.c", b"same content".as_slice()),
      ("to be modified.c", b"original content".as_slice()),
      ("to be removed.c", b"content to be removed".as_slice()),
    ],
    modified: vec![],
    removed: vec![],
  });
  
  // Add files
  fs::write(root.join("newly_added.c"), b"new content")?;
  let updates = Updates::new(root, watcher.poll_timeout(Duration::from_millis(2)))?;
  assert_eq!(updates, Updates{
    added: vec![
      ("newly_added.c", b"new content".as_slice()),
    ],
    modified: vec![],
    removed: vec![],
  });

  // Modify files
  fs::write(root.join("to be modified.c"), b"modified content")?;
  std::thread::sleep(Duration::from_millis(2));

  let updates = Updates::new(root, watcher.poll_timeout(Duration::from_millis(2)))?;
  assert_eq!(updates, Updates{
    added: vec![],
    modified: vec![
      ("to be modified.c", b"modified content".as_slice()),
    ],
    removed: vec![],
  });

  // Remove files
  fs::remove_file(root.join("to be removed.c"))?;

  let updates = Updates::new(root, watcher.poll_timeout(Duration::from_millis(2)))?;
  assert_eq!(updates, Updates{
    added: vec![],
    modified: vec![],
    removed: vec![
      "to be removed.c",
    ],
  });

  // Rename files
  fs::rename(root.join("newly_added.c"), root.join("just renamed.c"))?;

  std::thread::sleep(Duration::from_millis(2)); // TODO: is there a better way
  let updates = Updates::new(root, watcher.poll_timeout(Duration::from_millis(2)))?;
  assert_eq!(updates, Updates{
    added: vec![("just renamed.c", b"new content")],
    modified: vec![],
    removed: vec!["newly_added.c"],
  });

  // Move files out of the watched dir
  let second_tmp_dir = TempDir::new("second_tmp_dir").unwrap();
  let root_2 = second_tmp_dir.path();
  fs::rename(root.join("just renamed.c"), root_2.join("just moved out.c"))?;

  let updates = Updates::new(root, watcher.poll_timeout(Duration::from_millis(2)))?;
  assert_eq!(updates, Updates{
    added: vec![],
    modified: vec![],
    removed: vec!["just renamed.c"],
  });

  // Move files into the watched dir
  fs::rename(root_2.join("just moved out.c"), root.join("moved back in.c"))?;

  let updates = Updates::new(root, watcher.poll_timeout(Duration::from_millis(2)))?;
  assert_eq!(updates, Updates{
    added: vec![("moved back in.c", b"new content")],
    modified: vec![],
    removed: vec![],
  });
  
  // Ignore files that aren't c files
  fs::write(root.join("to be ignored"), b"still ignored")?;
  let updates = Updates::new(root, watcher.poll_timeout(Duration::from_millis(2)))?;
  assert_eq!(updates, Updates{
    added: vec![],
    modified: vec![],
    removed: vec![],
  });

  fs::rename(root.join("to be ignored"), root.join("still to be ignored"))?;
  let updates = Updates::new(root, watcher.poll_timeout(Duration::from_millis(2)))?;
  assert_eq!(updates, Updates{
    added: vec![],
    modified: vec![],
    removed: vec![],
  });

  fs::rename(root.join("still to be ignored"), root_2.join("OUTSIDE"))?;
  let updates = Updates::new(root, watcher.poll_timeout(Duration::from_millis(2)))?;
  assert_eq!(updates, Updates{
    added: vec![],
    modified: vec![],
    removed: vec![],
  });

  fs::rename(root_2.join("OUTSIDE"), root.join("still to be ignored"))?;
  let updates = Updates::new(root, watcher.poll_timeout(Duration::from_millis(2)))?;
  assert_eq!(updates, Updates{
    added: vec![],
    modified: vec![],
    removed: vec![],
  });

  fs::remove_file(root.join("still to be ignored"))?;
  let updates = Updates::new(root, watcher.poll_timeout(Duration::from_millis(2)))?;
  assert_eq!(updates, Updates{
    added: vec![],
    modified: vec![],
    removed: vec![],
  });

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

#[derive(Default, Debug, PartialEq, Eq)]
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

    updates.added.sort();
    updates.modified.sort();
    updates.removed.sort();

    return Ok(updates);
  }
}