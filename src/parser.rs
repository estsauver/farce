pub trait Parser {
  fn apply<'b>(&self, input: &'b str) -> Option<&'b str>;
}

pub struct LiteralParser<'a> { literal: &'a str }

impl <'a> Parser for LiteralParser<'a> {
  fn apply<'i>(&self, input:&'i str) -> Option<&'i str> {
    // Check for case where input isn't as long as literal
    // zipper's will terminate as soon as 
    // TODO are there cases where the unicode representation
    // of one is different size but they incode the same size?
    if input.len() < self.literal.len() {
      return None
    }

    let zip_iter = self.literal.chars().zip(input.chars());
    match zip_iter.all(|&: (x, y)| { x == y }) {
      true => return Some(&input[..self.literal.len()]),
      false => return None
    }
  } 

//fn either(&self, other: Parser) -> Parser {
//}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_literal_parser() {
    let good_input = "Earl is getting rusty".as_slice();
    let earl_matcher = LiteralParser{ literal: "Earl".as_slice()};
    assert!(earl_matcher.apply(good_input).is_some());
    let bad_input = "This isn't early".as_slice();
    assert!(earl_matcher.apply(bad_input).is_none());
  }


  #[test]
  fn test_input_shorter_than_literal() {
    let lit = "one".as_slice();
    let input = "on".as_slice();
    assert!(LiteralParser { literal: lit }.apply(input).is_none());
  }
}
