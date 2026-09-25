use std::io;


pub fn input() -> String{
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro de entrada");
    let entrada = entrada.trim();

    entrada.to_string()
}

pub fn io_i32() {
    let mut entrada = String::new();
}