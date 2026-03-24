use std::io;

fn main(){

    let mut input = String::new();

    println!("Insira um numero: ");
    io::stdin().read_line(&mut input).expect("");

    let input: f32 = input.trim().parse().expect("");

    let km = input/1000.0;
    let cm = input*100.0;
    let mm = input*1000.0;
    println!("Em metros: {input}\nEm km: {km}\nEm cm: {cm}\nEm mm: {mm}");

}
