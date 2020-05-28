// package rustwebservice, module helloworld
// run unit test: cd <project-root-directory> && cargo test

// return "Hello World" String. Served as main service of this application.
pub fn helloworld() -> String {
  String::from("Hello World")
}

// #[cfg(test)] is attribute marker for test module
#[cfg(test)]
mod tests {
  // importing names from outer (for mod tests) scope.
  use super::*;

  // #[test] is attribute marker for test function
  #[test]
  fn test_helloworld() {
    assert_eq!(helloworld(), String::from("Hello World"));
  }
}
