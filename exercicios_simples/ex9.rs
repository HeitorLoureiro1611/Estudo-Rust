use std::io;

fn main(){
    let mut input = String::new();

    println!("Insira um valor em reais: ");

    io::stdin().read_line(&mut input).expect("Falha");
    let input: f32 = input.trim().parse().expect("Falha");
    let dolar = input * 5.28;

    println!("o valor R${input} em dolares é: ${:.2}", dolar);
}
