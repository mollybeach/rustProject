// Lesson #28
fn main() -> f32 {
    24.5
  }
  fn output(_first_num: i8, _operator: char, _second_num: i8,  _fourth: i8 ){
    
  }
  
  #[cfg(test)]
  mod tests {
    use crate::main;
    use crate::output;
  
    #[test]
    #[should_panic]
    fn main_panics_with_i() {
      assert_eq!(main() as usize as f32, main() as f32);
    }
    #[test]
    fn main_returns_f() {
      assert_eq!(main() as f32, 24.5);
    }
  
    #[test]
    fn output_expects_four_args() {
      let out = output(-10, '+', 10, 0);
      assert_eq!(out, ());
    }
  }
  
// Lesson #30
fn main() {

}

fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

#[cfg(test)]
mod tests {
  use crate::main;
  use crate::output;
  #[test]
  fn main_returns_empty_tuple() {
    assert_eq!(main(), ());
  }

  #[test]
  fn output_expects_four_args() {
    let out = output(-10, '+', 10, 0);
    assert_eq!(out, String::from("-10 + 10 = 0"));
  }
}

// Lesson #30
fn main() {
    println!("{}", output(2,'+', 2, 4));
   
  }
  
  fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
    format!(
      "{} {} {} = {}",
      first_number, operator, second_number, result
    )
  }
  
  #[cfg(test)]
  mod tests {
    use crate::main;
    use crate::output;
    #[test]
    fn main_returns_empty_tuple() {
      assert_eq!(main(), ());
    }
  
    #[test]
    fn output_expects_four_args() {
      let out = output(-10, '+', 10, 0);
      assert_eq!(out, String::from("-10 + 10 = 0"));
    }
  }
  
  // Lesson #32
fn main() {
    let first_number = 1;
    let operator = '-';
    let second_number = 10;
  
    println!("{}", output(first_number, operator, second_number, 0));
  }
  
  fn output(first_number: i32, operator: char, second_number: i32, result: i32) -> String {
    format!(
      "{} {} {} = {}",
      first_number, operator, second_number, result
    )
  }
  
  
  
  #[cfg(test)]
  mod tests {
    use crate::main;
    use crate::output;
  
    #[test]
    fn main_returns_empty_tuple() {
      assert_eq!(main(), ());
    }
  
    #[test]
    fn output_expects_four_args() {
      let out = output(-10, '+', 10, 0);
      assert_eq!(out, String::from("-10 + 10 = 0"));
    }
  
    #[test]
    fn operate_expects_three_args() {
      let op = operate('-', -5, 200);
      assert_eq!(op, ());
    }
  }
  
  