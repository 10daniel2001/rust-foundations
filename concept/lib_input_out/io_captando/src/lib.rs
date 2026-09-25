use std::io;


pub fn input() -> String{
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro de entrada");
    let entrada = entrada.trim();

    entrada.to_string()
}

pub fn io_i32() -> i32 {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro de numero");
    let _eentrada: i32 = entrada.trim().parse().expect("Erro de saida");

    _eentrada
}