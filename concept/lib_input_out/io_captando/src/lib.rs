use std::io;
use std::str::FromStr;
use std::fmt::Debug;



pub fn input() -> String{
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro de entrada");
    let entrada = entrada.trim();

    entrada.to_string()
}

pub fn io_i32() -> i32 {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro de numero");
    let _eentrada = match entrada.trim().parse() {
        Ok(numero) => numero,
        Err(_) => {
            println!("Erro ao associar numero");
            0
        }
    };

    _eentrada
}

pub fn io_char() -> char {
    let mut _entrada = String::new();
    io::stdin().read_line(&mut _entrada).expect("Erro de entrada");
   
    let character = match _entrada.trim().chars().next() {
        Some(c) => c,
        None => {
            println!("ERRO nenhum caracter");
            ' '
        }
    };

    character
}

//*****************************************************************************************
// A BAIXO ESTA UMA FUNÇAO DIFERNETE ONDE NAO SABEMOS O TYPE DO DADOS /
//
pub fn io_valor<T: FromStr>(mensagem: &str) -> T
where
    T::Err: Debug,
{
    let mut entrada = String::new();
    println!("{}", mensagem);
    io::stdin().read_line(&mut entrada).expect("Erro de leitura");

    entrada.trim().parse().expect("Entrada inválida")
}