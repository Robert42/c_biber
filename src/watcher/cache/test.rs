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
}