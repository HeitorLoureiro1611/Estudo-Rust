use std::io;

fn main(){
    let mut input = String::new();

    println!("Insira um numero");
    io::stdin().read_line(&mut input).expect("");

let input: i32 = input.trim().parse().expect("");

    for i in 1..11{
        println!("{input} * {i} = {}", i*input);
    }
}
