use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");
    println!("Please enter an expression to evaluate:");
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
      Ok(_) => {
          let result = calculate(&input);
          match result {
            Some(val) => println!("Result: {}", val * 24),
            None => println!("Invalid expression"),
          }
        }
        Err(error) => println!("error: {error}"),
      }
    }
    
    
fn calculate(input: &str) -> Option<i32> {
  let result = input.trim().parse().ok();
  result
}

// find the symbols in the input string
// coninputVec`ert the string numbers to numbers

// https://github.com/andars/rust-calculator

// *  Result -> Ok() or Err() <- Panic

// * expect() - like Result but you can specific the error message
// s.parse().expect("error message")

// * unwrap() - you agree that your program will crash if there is an error. Good for prototyping.
// s.parse().unwrap()

// * unwrap_or() - if there is an error, use the value passed into unwrap_or() instead of panicing.
// * unwrap_or() - works on Result or works on Option so you have to be very careful you are using it on the right type.
// s.parse().unwrap_or(0) <- on a Result
// to_int(&s).unwrap_or(0) <- on an Option

// Converting between Result and Option
// fn to_int(s: &str) -> Option<i32> {
//   match s.parse() {
//     Ok(n) => Some(n),
//     Err(_) => None,
//   }
// }

// ORRR

// fn to_int_2(s: &str) -> Option<i32> {
//   s.parse().ok()
// }

// * ok() - converts between Result and Option
// * ok_or(ErrorType) - converts an Option to a Result and you can specify the error message

// * Ok() - is a successful variant of Result

// * .map_err() - converts to the same container but with a different type

// * ? operator is used to propagate the absence of a value or a success up the stack