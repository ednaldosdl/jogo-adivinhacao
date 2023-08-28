use std::io;

fn convert_to_int(data_input: & String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}
fn main() {
  // Desafio é criar um código usando while que o usuário coloca de entrada um valor, exemplo 25 e a saída some (2+5) e o resultado seja 7.
  println!("Digite um número");
  let mut entrada = String::new();
  io::stdin().read_line(&mut entrada).expect("Atenção: Falha ao ler a entarda.");

  let n = convert_to_int(&entrada);
    let mut valor = n;
    let mut soma = 0;

    while valor > 0 {
      let digito = valor % 10;
      soma += digito;
      valor /= 10;
    }
    println!("A soma dos digitos de {} é: {}", n, soma);
}



// use std::io;

// fn main() {
//   // Desafio é criar um código usando while que o usuário coloca de entrada um valor, exemplo 25 e a saída some (2+5) e o resultado seja 7.
//   println!("Digite um número");
//   let mut entrada = String::new();
//   io::stdin().read_line(&mut entrada).expect("Atenção: Falha ao ler a entarda.");

//   let numero: u32 = entrada.trim().parse().expect("Digite um número válido");
//     let mut valor = numero;
//     let mut soma = 0;

//     while valor > 0 {
//         let digito = valor % 10;
//         soma += digito;
//         valor /= 10;
//     }
//     println!("A soma dos dígitos de {} é: {}", numero, soma);
// }