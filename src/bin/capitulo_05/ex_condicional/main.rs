
fn numero_maior_que() {
  //  Comparar número maior que o outro.
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

fn comparar_numero() {
  //  Comparar número com if, && ou || e os operadores.
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

fn comparar_numero_String() {
  //  Comparar números e string com if, && ou || e os operadores.
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

fn main() {

  println!("Trabalhando com operadores, condicionais e string.");

  numero_maior_que();
  comparar_numero();
  comparar_numero_String();

}