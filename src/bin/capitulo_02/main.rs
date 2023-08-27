
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    //  Criando o fammoso jogo de Adivinhar o Número.
    println!("Adivinhe o núemro!");

    let valor_secreto: u32 = rand::thread_rng().gen_range(1..101);
    println!("O número secreto é: {}", valor_secreto);

    loop {    
        println!("Por favor informe um número.");
        let mut adivinhar: String = String::new();
        io::stdin().read_line(&mut adivinhar).expect("Falha ao ler a entrada");

        println!("você adivinhou: {adivinhar}");

        let adivinhar:u32 = match adivinhar.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Seu palpite: {}", adivinhar);    
        match adivinhar.cmp(&valor_secreto) {
            Ordering::Less => println!("Muito pequeno!"),
            Ordering::Greater => println!("Muito grande!"),
            Ordering::Equal => {
                println!("Você ganhou!");
                break;
            }
        }
    }
}
