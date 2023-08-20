use std::io;

fn main() {

  println!("Informe seu nome.");
  let mut name = String::new();
  io::stdin().read_line(&mut name).expect("Falha ao ler entrada");

  println!("Olá, {}!", name.trim());
}