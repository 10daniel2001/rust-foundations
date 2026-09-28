struct Usuario {
    nome: String,
    idade: u32,
    ativo: bool,
}

fn main(){

    let us = Usuario {
        nome: String::from("Carlos daniel"),
        idade: 25,
        ativo: true,
    };

    println!("Nome {} - Idade {} - ativo {}", us.nome, us.idade, us.ativo);
}