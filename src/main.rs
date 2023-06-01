use Digit::*;
fn main() {
  println!("{}", get_lcd(1));
}

const ONE: &str = r#"
   
  |
  |
"#;

const TWO: &str = r#"
 _ 
 _|
|_ 
"#;

const THREE: &str = r#"
 _ 
 _|
 _|
"#;

const FOUR: &str = r#"
   
|_|
  |
"#;

const FIVE: &str = r#"
 _ 
|_ 
 _|
"#;

const SIX: &str = r#"
 _ 
|_ 
|_|
"#;

const SEVEN: &str = r#"
 _ 
  |
  |
"#;

const HEIGHT: &str = r#"
 _ 
|_|
|_|
"#;

const NINE: &str = r#"
 _ 
|_|
 _|
"#;

const ZERO: &str = r#"
 _ 
| |
|_|
"#;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Digit {
  _0,
  _1,
  _2,
  _3,
  _4,
  _5,
  _6,
  _7,
  _8,
  _9,
}

fn convert_to_digit(i: u128) -> Digit {
  match i {
    0 => _0,
    1 => _1,
    2 => _2,
    3 => _3,
    4 => _4,
    5 => _5,
    6 => _6,
    7 => _7,
    8 => _8,
    9 => _9,
    _ => unreachable!(),
  }
}
fn convert_to_digits(i: u128) -> Vec<Digit> {
  let mut quotient = i;
  let mut digits = vec![];
  loop {
    let remainder = quotient % 10;
    quotient = quotient / 10;
    digits.push(convert_to_digit(remainder));
    if quotient == 0 {
      break;
    }
  }
  digits.reverse();
  digits
}

fn get_lcd(i: u128) -> String {
  let digits = convert_to_digits(i);
  let digits = digits.into_iter().map(|d| get_lcd_digit(d)).collect::<Vec<_>>();
  let (line1, line2, line3) =
    digits
      .iter()
      .map(|f| f.lines().skip(1))
      .fold((vec![], vec![], vec![]), |(mut acc1, mut acc2, mut acc3), mut f| {
        acc1.push(f.next().unwrap_or_default());
        acc2.push(f.next().unwrap_or_default());
        acc3.push(f.next().unwrap_or_default());
        (acc1, acc2, acc3)
      });
  format!("\n{}\n{}\n{}\n", line1.join(""), line2.join(""), line3.join(""))
}

fn get_lcd_digit(digit: Digit) -> String {
  match digit {
    _1 => ONE,
    _2 => TWO,
    _3 => THREE,
    _4 => FOUR,
    _5 => FIVE,
    _6 => SIX,
    _7 => SEVEN,
    _8 => HEIGHT,
    _9 => NINE,
    _0 => ZERO,
  }
  .to_string()
}

// tests module
#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;
  use test_case::test_case;
  //use Digit::*;

  #[test_case(1, ONE)]
  #[test_case(2, TWO)]
  #[test_case(3, THREE)]
  #[test_case(4, FOUR)]
  #[test_case(5, FIVE)]
  #[test_case(6, SIX)]
  #[test_case(7, SEVEN)]
  #[test_case(8, HEIGHT)]
  #[test_case(9, NINE)]
  #[test_case(0, ZERO)]
  fn test(n: u128, expected: &str) {
    assert_eq!(get_lcd(n), expected);
  }

  #[test]
  fn the_test() {
    assert_eq!(
      get_lcd(1234567890),
      r#"
    _  _     _  _  _  _  _  _ 
  | _| _||_||_ |_   ||_||_|| |
  ||_  _|  | _||_|  ||_| _||_|
"#
    );
  }

  #[test]
  fn convert_1_to_digit_1() {
    assert_eq!(convert_to_digits(1), vec![_1]);
  }
  #[test]
  fn convert_12_to_digit_1_2() {
    assert_eq!(convert_to_digits(12), vec![_1, _2]);
  }
  #[test]
  fn convert_1024_to_digit_1_0_2_4() {
    assert_eq!(convert_to_digits(1024), vec![_1, _0, _2, _4]);
  }
}
