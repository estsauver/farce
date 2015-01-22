// Since there isn't a natural way to define these in tersm of the traits
// that they operate on, we define this Parser directly over &str since that's
// what we're interested in.
trait Parsers<Elem> {
  
}


#[derive(PartialEq)]
enum ParseResult<T> {
  Success(T),
  Failure,
  Error
}

impl <T> ParseResult<T> {
  fn map<U, F: Fn(&T) -> U>(&self, f: F) -> ParseResult<U> {
    match *self {
      ParseResult::Success(ref t) => ParseResult::Success(f(t)),
      ParseResult::Failure => ParseResult::Failure,
      ParseResult::Error => ParseResult::Error
    }
  }

}

#[cfg(test)] 
mod tests {
  use super::*;
  use super::ParseResult::*;

  #[test]
  fn test_mapSuccess() {
    let f = |&: x:&i32| { 2 * *x };
    let start = Success(1232);
    assert!(start.map(f) == Success(2464));
  }

  #[test]
  fn test_mapFailure() {
    let f = |&: x:&i32| { 2 * *x };
    let start = Failure;
    assert!(start.map(f) == Failure)
  }


  #[test]
  fn test_mapError() {
    let f = |&: x:&i32| { 2 * *x };
    let start = Error;
    assert!(start.map(f) == Error)
  }

}



