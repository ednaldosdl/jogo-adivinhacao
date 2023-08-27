
fn main() {
  let number1 = 5;
  let number2 = 10;
  let number3 = 15;

  let result = if number1 > number2 {
      number1
  } else if number2 > number3 {
      number2
  } else {
      number3
  };

  println!("Resultado é: {}", result);
}
