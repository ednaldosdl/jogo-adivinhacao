use std::io;

fn convert_to_int(data_input: & String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}

fn main() {
  // O desafio é criar um código para saber qual o calculo FATRIAL, usando entada de usuário.
  println!("Digite um número para calcular o fatorial:");
  let mut entrada_fatorial = String::new();
  io::stdin().read_line(&mut entrada_fatorial).expect("Atenção: Falha ao ler entrada.");

  let mut fatorial = 1;
  let mut entrada_int = convert_to_int(&entrada_fatorial);

  while entrada_int > 1 {
      fatorial = fatorial * entrada_int;
      entrada_int = entrada_int - 1;
  }
  println!("O fatorial é {}", fatorial);
}



// use std::io;

// fn convert_to_int(data_input: &str) -> i32 {
//     let x = data_input.trim().parse::<i32>().unwrap();
//     x
// }

// fn main() {
//     println!("Digite um número para calcular o fatorial:");

//     let mut entrada_fatorial = String::new();
//     io::stdin().read_line(&mut entrada_fatorial).expect("Atenção: Falha ao ler entrada.");

//     let entrada_int = convert_to_int(&entrada_fatorial);

//     if entrada_int >= 0 {
//         let mut fatorial = 1;
//         let mut contador = entrada_int;

//         while contador > 1 {
//             fatorial *= contador;
//             contador -= 1;
//         }

//         println!("O fatorial de {} é {}", entrada_int, fatorial);
//     } else {
//         println!("Não é possível calcular o fatorial de um número negativo.");
//     }
// }
