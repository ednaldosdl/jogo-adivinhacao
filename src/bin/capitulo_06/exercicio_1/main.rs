use std::io;

fn main() {
    println!("Digite as 4 notas do aluno:");

    let mut notas = Vec::new();

    for _ in 0..4 {
        let mut nota_input = String::new();
        io::stdin().read_line(&mut nota_input).expect("Falha ao ler a nota.");

        let nota: f64 = nota_input.trim().parse().expect("Digite um número válido.");
        notas.push(nota);
    }

    let soma: f64 = notas.iter().sum();
    let media = soma / 4.0;

    println!("A média do aluno é: {:.2}", media);
}
