
/*Estou criando uma forma de captar às entradas em rust de uma forma simples
Estou últilizando um arquivo lib

*/


use iocaptando::{input, io_i32};


fn main() {
    println!("Digite um texto");
    let texto = input();

    println!("Texto e {}", texto);

    println!("Digite um numero inteiro");
    let _numero = io_i32();
    println!("Numero e {_numero}");
}