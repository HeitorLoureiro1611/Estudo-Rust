use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    let numero = rand::thread_rng().gen_range(0..=100); //thread_rng cria uma seed
    let mut tentativas: i32 = 0;
    println!("Adivinhe o numero!!");
    println!("Insira um numero entre 1 e 100: ");

    loop {
        tentativas +=1;
        let mut chute = String::new(); //a variável mutável chute é do tipo string
        io::stdin()//metodo de leitura
            .read_line(&mut chute)
            .expect("Fail to read line!"); //lida com erros


        let chute: u32 = match chute.trim().parse(){ // trim tira os espaços e buffer, parse faz o cast de tipos
            Ok(num) => num,                          //parse retorna dois valores "Ok" e "Err"
            Err(_) => continue,      //lidando com inputs não esperados
        };


        println!("seu chute foi: {chute}");
        match chute.cmp(&numero){ //match é a comparação e cmp é o metodo pra lidar com as comparações
            Ordering::Less=>println!("O numero chutado foi menor"),
            Ordering::Greater=>println!("O numero chutado foi MAIOR"),
            Ordering::Equal=>{
                println!("Parabens, você acertou!");
                break;
            }
        }
    }
    println!("Você acertou em {tentativas} tentativas");
}
