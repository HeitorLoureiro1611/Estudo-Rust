struct Mensagem {
    texto: String,
}
impl Mensagem {
    fn ler(&self) {
        // apenas leitura
        println!("{}", self.texto)
    }
    fn editar(&mut self) {
        //alterar o valor
        self.texto = "Opa, editado".to_string()
    }
    fn consumir(self) {
        // consome o valor
        println!("O texto: {} agora não existirá mais!", self.texto);
    }
}
fn main() {
    // variável texto criada com o valor "olá"
    let mut texto = Mensagem {
        texto: "Olá!".to_string(),
    };
    //lê a variável
    texto.ler(); // -> OK
                 //muda a variável
    texto.editar(); // -> OK
                    //consome a variável e a lifetime para de existir
    texto.consumir(); // -> OK
                      // lê de novo a variável
                      //texto.ler(); -> ERRO! a variável foi consumida
}
