/*variaveis mutavel
  variavel mutavel é uma variavel que pode ser alterada durante a execução do programa, ou seja, seu valor pode ser modificado. Para declarar uma variavel mutavel em Rust, utilizamos a palavra-chave "mut" antes do nome da variavel. Por exemplo:
  
  EN:
    Mutable variables
    A mutable variable is a variable that can be changed during the execution of the program, meaning its value can be modified. To declare a mutable variable in Rust, we use the keyword "mut" before the variable name. For example:
*/

// Declarando uma funçao main, funçâo principal em rust
fn main() {
    let iidade = 25;
    // Idade e um inteiro de 32 bits aútomatico, mas o valor nao pode ser alterado esta fixado, mas é uma variavel presente na stack
    //idade = 99;
    // Certamente é um erro 
    // Ao compilar ou ao tentar compilar será gerando mensagens de erro

    // JEITO CORRETO

    let mut year = 2026;

    year = 2027;

    println!("{year}"); 
}