use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    let mut chute = String::new();
    let numero = rand::thread_rng().gen_range(0..=100);

    println!("Adivinhe o numero!!");

    println!("{numero}");
    println!("Insira um numero entre 1 e 100: ");

    io::stdin()
        .read_line(&mut chute)
        .expect("Fail to read line!");

    let chute: u32 = chute.trim().parse().expect("Insira um numero!");


    println!("seu chute foi: {chute}");
    match chute.cmp(&numero){
        Ordering::Less=>println!("O numero chutado foi menor"),
        Ordering::Greater=>println!("O numero chutado foi MAIOR"),
        Ordering::Equal=>println!("Parabens, você acertou!"),
    }
}
