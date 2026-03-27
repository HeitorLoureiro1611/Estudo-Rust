// Definição da struct contador com valor
struct Contador {
    valor: i32,
}
//implementação dos metodos pra esse struct
impl Contador {
    //metodo de incrementar 1 no valor
    // &mut self pois vai receber um valor e modificar ele
    fn incrementar(&mut self) {
        self.valor += 1;
    }
    //metodo de mostrar o valor formatado
    fn mostrar(&self) {
        // não vou precisa alterar o valor recebido
        println!("Contador está em: {}", self.valor);
    }
}
fn main() {
    //criação da variável que contém o valor do tipo Contador
    let mut cont = Contador { valor: 0 };

    for _n in 1..=5 {
        //uso dos metodos que fazem parte do tipo
        cont.incrementar();
        cont.mostrar();
    }
}
