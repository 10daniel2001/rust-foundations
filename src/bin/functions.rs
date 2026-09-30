/*Exemplos de funçoes e seus retornos 
*/


// Função que recebe dois parametros e retorna a soma deles
// return type é i32, ou seja, inteiro de 32 bits
fn somar(a: i32, b: i32) -> i32{
    return a + b;
}

// Função que recebe dois parametros e retorna a multiplicação deles
// Não é necessário o return, pois a ultima linha da função é o valor de retorno
fn multiplicar(a: f32, b: f32) -> f32{
    a * b
}

fn dividir(a: usize, b: usize) -> usize{
   return a / b
}



fn main() {
    let resultado = somar(10, 10);
    println!("Resultado {}", resultado);

    let res_multi = multiplicar(2.0, 10.0);
    println!("resultado de * é = {:?}", res_multi);

    let divisao = dividir(300, 20);
    println!("Resultado da divisao e {}", divisao);
}