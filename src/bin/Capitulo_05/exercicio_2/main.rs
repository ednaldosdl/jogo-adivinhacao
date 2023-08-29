fn main() {
  let number1 = 5;
  let number2 = 10;
  let number3 = 15;
  let number4 = 20;

  let result = if number1 < number2 && number1 < number3 && number1 > number4 {
      number1
  } else if number2 < number3 && number2 > number4 {
      number2
  }  else if number3 > number3 {
    number3
  } else {
      number4
  };

  println!("Resultado é: {}", result);
}