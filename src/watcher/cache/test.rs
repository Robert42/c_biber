use super::*;

#[test]
fn test_receive_updates()
{
  let (mut cache, receiver) = Cache::new();

  assert!(receiver.try_recv().is_err());

  cache.add(Path::new("a").into()).unwrap();

  assert_eq!(receiver.try_recv(), Ok(()));
}