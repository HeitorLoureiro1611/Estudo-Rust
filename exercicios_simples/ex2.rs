use std::io;

fn main(){
    println!("Digite seu nome: ");

    let mut nome = String::new();

    io::stdin().read_line(&mut nome)
       .expect("Não foi possivel ler o input");

    println!("Seu nome é: {nome}")
}
