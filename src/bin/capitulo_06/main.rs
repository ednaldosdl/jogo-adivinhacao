//extern crate rand;

use std::io;
//use std::cmp::Ordering;
//use rand::Rng;

fn main() {
  //  trabalhando com entrada para usuário.
  println!("Informe seu nome");
  let mut name = String::new();
  io::stdin().read_line(&mut name).expect("Falha ao ler a entrada.");

  println!("Olá, {}!", name.trim());

}