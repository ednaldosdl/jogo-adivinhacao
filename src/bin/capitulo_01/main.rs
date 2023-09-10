fn var_texto() {
  //  trabalhando com as variaveis String.
  let mut name = "Emilly";
  name = "Luiz Otávio";
  println!("Olá, {}!", name);
}

fn var_int() {
  //  trabalhando com as variaveis Inteiro.
  let x: i32 = 50;
  println!("O valor de x é: {}", x);
  
  let x = 60;
  println!("O valor de x é: {}", x);

  let y = x * 2;
  println!("O valor de y é: {}", y);
}

fn var_flut() {
  //  trabalhando com as variaveis Flutuante e constante.
  const PI: f32 = 3.141592;
  println!("O valor de PI é: {}", PI);
}

fn saudacoes() {
    let  nome = "Edinlado Soares";
    println!("Seja Bem-vindo, {}!", nome);
}

fn main() {
//  Iniciando com o famoso Hello World!.
  println!("Olá, Mundo!");
  saudacoes();
  var_texto();
  var_int();
  var_flut();
    
}
