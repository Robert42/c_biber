use super::*;

#[test]
fn test_receive_updates()
{
  let (mut cache, receiver) = Cache::new();

  assert!(receiver.try_recv().is_err());

  cache.add(Path::new("a"), vec![]).unwrap();

  assert_eq!(receiver.try_recv(), Ok(Event::ADD(Arc::from(Path::new("a")), vec![])));
  assert!(receiver.try_recv().is_err());

  cache.add(Path::new("a"), vec![]).unwrap();
  cache.add(Path::new("b"), vec![]).unwrap();

  assert_eq!(receiver.try_recv(), Ok(Event::ADD(Arc::from(Path::new("b")), vec![])));
  assert!(receiver.try_recv().is_err());

  cache.add(Path::new("a"), vec![42]).unwrap();
  cache.add(Path::new("b"), vec![]).unwrap();

  assert_eq!(receiver.try_recv(), Ok(Event::MODIFIED(Arc::from(Path::new("a")), vec![42])));
  assert!(receiver.try_recv().is_err());
}

#[test]
fn test_full_rescan()
{
  let (mut cache, receiver) = Cache::new();

  cache.add(Path::new("original_unmodified"), b"same content").unwrap();
  cache.add(Path::new("modified"), b"original content").unwrap();

  assert_eq!(receiver.try_recv(), Ok(Event::ADD(Arc::from(Path::new("original_unmodified")), b"same content".into())));
  assert_eq!(receiver.try_recv(), Ok(Event::ADD(Arc::from(Path::new("modified")), b"original content".into())));
  assert!(receiver.try_recv().is_err());

  cache.full_scan(
    |cache|
    {
      cache.add(Path::new("original_unmodified"), b"same content")?;
      cache.add(Path::new("modified"), b"modified content")?;
      cache.add(Path::new("newly_added"), b"new content")?;
      Ok(())
    }
  ).unwrap();

  assert_eq!(receiver.try_recv(), Ok(Event::MODIFIED(Arc::from(Path::new("modified")), b"modified content".into())));
  assert_eq!(receiver.try_recv(), Ok(Event::ADD(Arc::from(Path::new("newly_added")), b"new content".into())));
  assert!(receiver.try_recv().is_err());
}