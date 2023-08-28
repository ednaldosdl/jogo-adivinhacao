use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}
fn main() {
  //  trabalhando com entrada ultilizando número inteiro.
  println!("Informe o número:");
  let mut num1 = String::new();
  io::stdin().read_line(&mut num1).expect("ATENÇÃO: Falha ao ler entrada.");
  
  let mut num2 = String::new();
  io::stdin().read_line(&mut num2).expect("ATENÇÃO: Falha ao ler entrada.");
  
  if convert_to_int(&num1) > convert_to_int(&num2) {
      println!("O número {} é maior que: {}", num1, num2)
    } else if convert_to_int(&num1) == convert_to_int(&num2) {
      println!("O número {} é igual a: {}", num1, num2)
    } else {
      println!("O número {} é menor que: {}", num1, num2)
    }
}




// use std::io;
// fn main() {
//   //  trabalhando com entrada ultilizando o nome de usuário.
//   println!("Informe seu nome");
//   let mut name = String::new();
//   io::stdin().read_line(&mut name).expect("Falha ao ler a entrada.");

//   println!("Olá, {}!", name.trim());

// }