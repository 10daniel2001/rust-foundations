
/*Estou criando uma forma de captar às entradas em rust de uma forma simples
Estou últilizando um arquivo lib

*/


use iocaptando::{input, io_i32, io_char};
// Notem  este "use" chamando a lib e suas funçoes 


fn main() {
    // Funçao que capta a entrada de STRING
    println!("Digite um texto");
    let texto = input();
    println!("Texto e {}", texto);

    // Funçao que capta a entrada de numeros INTEIROS
    println!("Digite um numero inteiro");
    let _numero = io_i32();
    println!("Numero e {_numero}");

    // Funçao que capta à entrada de um CHAR
    println!("Digite um caracter");
    let caracters = io_char();
    println!("Seu caracter e {caracters}");
}