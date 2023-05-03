#[test]
fn useless_test() {
  assert_eq!(true, true);
}

#[test]
#[ignore]
fn always_fail() {
  assert!(false);
}
