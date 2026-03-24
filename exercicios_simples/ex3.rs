use::std::io;

fn main(){

    //declaração das variaveis como String
    let mut n1 = String::new();
    let mut n2 = String::new();

    //recebimento dos valores
    println!("Insira um numero: ");
    io::stdin().read_line(&mut n1)
        .expect("Falha leitura n1");

    println!("Insira outro numero: ");
    io::stdin().read_line(&mut n2)
        .expect("Falha leitura n2");

    //passando os valores de string para numeros inteiros de 32 bits
    let n1: u32 = n1.trim().parse().expect("Não é um inteiro");
    let n2: u32 = n2.trim().parse().expect("Não é um inteiro");
    let soma = n1+n2;

    //retornando o valor da soma dos dois numeros
    println!("A soma dos dois valores é: {soma}");
}
