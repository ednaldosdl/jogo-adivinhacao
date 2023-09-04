use std::io;

fn main() {
    println!("Digite um número:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler a entrada");

    let number: i32 = input.trim().parse().expect("Digite um número válido!");

    if number % 2 == 0 {
        println!("O número {} é par.", number);
    } else {
        println!("O número {} é ímpar.", number);
    }
}