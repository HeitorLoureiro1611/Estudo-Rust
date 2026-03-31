enum DiasDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}
fn tipo_dia(dia: DiasDaSemana) -> &'static str {
    match dia {
        DiasDaSemana::Domingo | DiasDaSemana::Sabado => "Fim de semana",
        _ => "Dia da semana",
    }
}

fn main() {
    let dia = DiasDaSemana::Segunda;
    println!("Segunda: {}", tipo_dia(dia));
}
