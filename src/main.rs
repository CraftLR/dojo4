fn main() {
  println!("Hello, world!");
}

// tests module
#[cfg(test)]
mod tests {
  #[test]
  fn usually_rust_tests_are_just_in_the_same_file() {
    assert!(matches!(Some(true), Some(x) if x));
  }
}
