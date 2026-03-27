const PI: f64 = 3.1415;
//uso de struct
struct Circulo {
    raio: f64,
}
//implementação de um metodo pro struct
impl Circulo {
    //uso do &self significa que não quero consumir a variavel, quero que seja apenas um emprestimo
    // se fosse apenas self a variável teria o lifetime nessa função.
    fn area(&self) -> f64 {
        (self.raio * self.raio) * PI
    }
}
fn main() {
    //declaração de variavel do tipo do struct que recebe o valor na chave correspondente
    let r = Circulo { raio: 2.0 };
    //uso do metodo
    println!("A área do Circulo é: {}", r.area())
}
