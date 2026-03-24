use std::io;

fn main(){
    let mut numero = String::new();

    println!("Insira um numero: ");
    io::stdin().read_line(&mut numero).expect("Falha leitura");

    let numero: i32 = numero.trim().parse().expect("Falha");

    println!("{} + 1 = {}\n{} - 1 = {}", numero, numero+1, numero, numero-1);
}
