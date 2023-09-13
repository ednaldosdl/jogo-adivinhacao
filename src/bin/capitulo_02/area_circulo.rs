use std::io;

fn convert_to_float(data_input: &String) -> f64 {
    data_input.trim().parse().expect("Falha ao converter para f64")
}

fn main() {
    println!("\n__________________________________");
    println!("  CALCULADORA DE ÁREA DE CÍRCULO ");
    println!("==================================\n");

    // Solicita ao usuário o raio do círculo
    println!("Por favor, digite o raio do círculo: ");


    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Atenção: Falha ao ler entrada.");

    let raio: f64 = convert_to_float(&entrada);

    if raio.is_nan() {
        println!("Entrada inválida. Certifique-se de digitar um número válido.");
        return;
    }

    // Calcula a área do círculo
    let area = calcular_area(raio);

    // Exibe o resultado
    println!("A área do círculo com raio {} é: {}", raio, area);
}

fn calcular_area(raio: f64) -> f64 {
    // Fórmula para calcular a área do círculo: A = π * r^2
    const PI: f64 = 3.14159265359;
    PI * raio * raio
}
