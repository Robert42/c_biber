use super::*;

#[test]
fn find_all_c_files()
{
  assert_eq!(collect_cache([]).unwrap(), vec![] as Vec<String>);
}

fn create_files<'a, Files>(files: Files) -> Result<TempDir>
where
  Files: IntoIterator<Item=(&'a str, &'a [u8])>
{
  let tmp_dir = TempDir::new("find_all_c_files").unwrap();

  Ok(tmp_dir)
}

fn collect_cache<'a, Files>(files: Files) -> Result<Vec<String>>
where
  Files: IntoIterator<Item=(&'a str, &'a [u8])>
{
  let tmp_dir = create_files(files)?;

  let mut watcher = Watcher::new(tmp_dir.path(), |_| Some(true));
  watcher.scan()?;

  Ok(watcher.cache.into_iter().map(|x| format!("{}", x.display())).collect())
}