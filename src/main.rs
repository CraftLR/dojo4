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
  get_lcd_digit(digits[0])
}

fn get_lcd_digit(digit: Digit) -> String {
  match digit {
    _1 => ONE,
    _2 => TWO,
    _ => THREE,
  }
  .to_string()
}

// tests module
#[cfg(test)]
mod tests {
  use super::*;
  //use Digit::*;

  #[test]
  fn test_1() {
    assert_eq!(get_lcd(1), ONE);
  }
  #[test]
  fn test_2() {
    assert_eq!(get_lcd(2), TWO);
  }
  #[test]
  fn test_3() {
    assert_eq!(get_lcd(3), THREE);
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
