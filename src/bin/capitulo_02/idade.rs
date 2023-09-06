
use std::io;

fn main() {
  // Crie um programa que entre com o ano de nascimento e imprima o nome e a idade do usuário (ideia de exercício próprio).
  let nome = "Edinlado Soares";
  println!("\nInforma o ano de nascimento: ");
  const ANO_ATUAL: i32 = 2023;
  let mut ano = String::new();
  io::stdin().read_line(&mut ano).expect("Atenção: Falha ao ler entrada.");

  let ano_nascimento: i32 = ano.trim().parse().expect("Erro ao converter ano para número");
  let idade = ANO_ATUAL - ano_nascimento;
  println!("Olá, {}! \nSua idade é: {} anos.", nome, idade);
}