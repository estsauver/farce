// Since there isn't a natural way to define these in tersm of the traits
// that they operate on, we define this Parser directly over &str since that's
// what we're interested in.
trait Parsers<T> {
  fn apply(input: Input<T>) -> ParseResult<T, Input<T>>;
}

trait Input<A> : Iterator {}
impl <A, I:Iterator<A>> Input<A> for I {}

#[derive(PartialEq)]
enum ParseResult<Elem, T> {
  Success(T, Iterator<Elem>),
  Failure(Iterator<Elem>),
  Error
}

// 
impl <T> ParseResult<T> {
  fn map<U, F: Fn(T) -> U>(self, f: F) -> ParseResult<U> {
    match self {
      ParseResult::Success(t) => ParseResult::Success(f(t)),
      ParseResult::Failure => ParseResult::Failure,
      ParseResult::Error => ParseResult::Error
    }
  }

  fn append(self, x: ParseResult<T>) -> ParseResult<T> {
    match self {
      ParseResult::Success(_) => self,
      ParseResult::Failure => {
        match x {
          ParseResult::Success(_) => x,
          // TODO, actually need to return the parse result that has gone 
          // the furthest along the input.
          ParseResult::Failure => x,
          ParseResult::Error =>x
        }
      }
      ParseResult::Error => self
    }
  }
}

#[cfg(test)] 
mod tests {
  use super::*;
  use super::ParseResult::*;

  #[test]
  fn test_map_success() {
    let f = |&: x:i32| { 2 * x };
    let start = Success(1232);
    assert!(start.map(f) == Success(2464));
  }

  #[test]
  fn test_map_failure() {
    let f = |&: x:i32| { 2 * x };
    let start = Failure;
    assert!(start.map(f) == Failure)
  }


  #[test]
  fn test_map_error() {
    let f = |&: x:i32| { 2 * x };
    let start = Error;
    assert!(start.map(f) == Error)
  }
}



