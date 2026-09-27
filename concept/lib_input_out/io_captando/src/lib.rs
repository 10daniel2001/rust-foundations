use std::io;
use std::str::FromStr;
// biblioteca para tratar erros de conversão de string para outro tipo
use std::fmt::Debug;
// std::fmt::Debug é um trait que permite formatar valores para depuração. Ele é usado aqui para garantir que o tipo T possa ser depurado em caso de erro de conversão.



pub fn input() -> String{
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro de entrada");
    let entrada = entrada.trim();

    entrada.to_string()

    // Sobre está função: Ela lê uma linha de entrada do usuário, 
    // remove espaços em branco no início e no final da string e retorna a string resultante.
    // A função é útil para capturar entradas de texto do usuário de forma simples e direta.
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

    // A função io_i32 lê uma linha de entrada do usuário, tenta converter a entrada para um número
    // inteiro (i32) e retorna o valor resultante.
    // Se a conversão falhar, a função imprime uma mensagem de erro e retorna 0 como valor padrão.
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