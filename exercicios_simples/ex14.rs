enum Pagamento {
    Dinheiro(f64),
    Cartao(f64, String),
    Pix(f64, String),
}
fn processar(forma: Pagamento) {
    match forma {
        Pagamento::Dinheiro(valor) => println!("O valor em dinheiro é: R${}", valor),
        Pagamento::Cartao(valor, titular) => println!("O valor é: R${} do {}", valor, titular),
        Pagamento::Pix(valor, chave) => println!("O valor é: R${} da chave: {}", valor, chave),
    }
}
fn main() {
    let pagamento_din = Pagamento::Dinheiro(50.0);
    let pagamento_card = Pagamento::Cartao(299.99, "Heitor Loureiro".to_string());
    let pagamento_pix = Pagamento::Pix(5.30, "49327890478".to_string());
    processar(pagamento_din);
    processar(pagamento_card);
    processar(pagamento_pix);
}
