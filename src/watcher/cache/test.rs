use super::*;

#[test]
fn test_receive_updates()
{
  let (mut cache, receiver) = Cache::new();

  assert!(receiver.try_recv().is_err());

  cache.add(Path::new("a")).unwrap();

  assert_eq!(receiver.try_recv(), Ok(Arc::from(Path::new("a"))));
  assert!(receiver.try_recv().is_err());

  cache.add(Path::new("a")).unwrap();
  cache.add(Path::new("b")).unwrap();

  assert_eq!(receiver.try_recv(), Ok(Arc::from(Path::new("b"))));
  assert!(receiver.try_recv().is_err());
}