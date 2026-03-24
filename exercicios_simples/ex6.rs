use std::io;

fn main(){

    let mut n1 = String::new();
    let mut n2 = 0;

    println!("Insira a primeira nota do aluno: ");
    io::stdin().read_line(&mut n1).expect("Falha leitura n1");

    println!("Insira a segunda nota do aluno: ");
    io::stdin().read_line(&mut n2).expect("Falha leitura n2");

    let n1: f32 = n1.trim().parse().expect("Precisa ser um numero");
    let n2: f32 = n2.trim().parse().expect("Precisa ser um numero");

    let media = (n1 + n2)/2.0;

    println!("A média do aluno é: {media}");
}
