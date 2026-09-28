
/*
As coleções principais (std::collections)
Estrutura	Uso
Vec<T>	lista dinâmica, crescimento no heap (o "array" do dia a dia)
String	texto UTF-8 dinâmico 
HashMap<K, V>	chave → valor, tipo dicionário
HashSet<T>	conjunto sem duplicatas
VecDeque<T>	fila dupla (push/pop nas duas pontas)
BTreeMap<K, V>	como HashMap mas ordenado por chave
 */




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