use std::io;

fn main() {

  println!("Informe as notas: ");

  let mut notas = Vec::new();

  for _ in 0..3 {
    let mut nota_input = String::new();
    io::stdin().read_line(&mut nota_input).expect("Atenção: Falha ao ler entrada.");

    let nota: f64 = nota_input.trim().parse().expect("Insira uma nota válida.");
    notas.push(nota);
  }  

    let soma: f64 = notas.iter().sum();
    let media = soma / 3.0;

    println!("A média do aluno é: {:.2}", media);

}