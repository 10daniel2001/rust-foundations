/* Em rust todas as variaveis são imutaveis. Mas ao adicionar mut ela se torna mútavel


*/

fn testing(){

    // let numero = 256; // Número inteiro 

    // numero = 555; // Erro isso nao vai rodar está variavel nao é mútavel !


    let mut numbers = 256;
    // Valor original é vai ser movido
    // Ao adicionar o mut, está variavel e mútavel, pode haver mudança de valores 
    numbers = 555; // Novo valor atribuido, corretamente 


    println!("{}", numbers); // Macro para saida de dados
}

fn main() {
    
    // Chamando a funçâo 
    testing();
}