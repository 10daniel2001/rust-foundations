use std::io;


pub fn input() -> String{
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro de entrada");
    let entrada = entrada.trim();

    entrada.to_string()
}