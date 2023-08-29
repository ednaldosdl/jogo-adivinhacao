fn main() {
  let number1 = 5;
  let number2 = 10;
  let string1 = "hello";
  let string2 = "world";
  let boolean1 = true;
  let boolean2 = false;

  let result = if boolean1 && (number1 > number2 || string1 == "hello") {
      number1
  } else if boolean2 || (number2 > number1 && string2 != "world") {
      number2
  }  else {
    number1 + number2
  };

  println!("Resultado é: {}", result);
}