use std::io;

fn main() {
    //quando eu estiver manipulando o vetor eu tiro o comentario
    //let mut lista: Vec<String> = Vec::new();
    let mut finalizar: bool = true;
    while finalizar {
        let mut op = String::new();

        println!("\nMenu do programa!\n");
        println!("======================");
        println!("[1] Adicionar Tarefa");
        println!("[2] Mostrar lista");
        println!("[3] Atualizar Tarefa");
        println!("[4] Remover Tarefa");
        println!("[9] Finalizar programa");
        println!("======================");

        println!("\nInsira opção:");
        io::stdin().read_line(&mut op).expect("Falha no input");
        let mut op: u32 = op.trim().parse().expect("Deve ser um numero");

        match &mut op {
            1 => println!("Adicionando"),
            2 => println!("Mostrando"),
            3 => println!("Atualizando"),
            4 => println!("Removendo"),
            9 => {
                println!("Até mais!");
                finalizar = false
            }
            _ => println!("Input inválido"),
        }
        println!("======================");
    }
}
