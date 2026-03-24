use std::io;

fn main(){
    let mut numero = String::new();

    println!("Insira um numero: ");
    io::stdin().read_line(&mut numero).expect("Falha, stdin");
    let numero: i32 = numero.trim().parse().expect("Falha, numero");

    println!("O dobro: {}", numero*2);
    println!("O triplo: {}", numero*3);
    println!("A raiz quadrada: {}", numero*numero);
}
