use super::*;

#[test]
fn expected_keyword() {
  let test = Test::new();
  let path = test.justfile_path();

  test
    .justfile("foo := if '' == '' { '' } arlo { '' }")
    .stderr(format!(
      "
    error: Expected keyword `else` but found identifier `arlo`
      |
    1 | foo := if '' == '' {{ '' }} arlo {{ '' }}
      |                           ^^^^
      --> {}:1:26
  ",
      path.to_str().unwrap()
    ))
    .status(EXIT_FAILURE)
    .run();
}

#[test]
fn unexpected_character() {
  let test = Test::new();
  let path = test.justfile_path();

  test
    .justfile("!~")
    .stderr(format!(
      "
    error: Expected character `=`
      |
    1 | !~
      |  ^
      --> {}:1:2
  ",
      path.to_str().unwrap(),
    ))
    .status(EXIT_FAILURE)
    .run();
}

#[test]
fn argument_count_mismatch() {
  let test = Test::new();
  let path = test.justfile_path();

  // TODO how should this be displayed?
  test
    .justfile("foo a b:")
    .stderr(format!(
      "
      error: Recipe `foo` got 0 arguments but takes 2
      usage:
          just foo a bench
    {}
    ",
      path.to_str().unwrap()
    ))
    .status(EXIT_FAILURE)
    .run();
}
