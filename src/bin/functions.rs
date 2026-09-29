/*Exemplos de funçoes e seus retornos 
*/

fn somar(a: i32, b: i32) -> i32{
    return a + b;
}
fn multiplicar(a: f32, b: f32) -> f32{
    a * b
}



fn main() {
    let rseultado = somar(10, 10);
    println!("Resultado {}", rseultado);

    let res_multi = multiplicar(2.0, 10.0);
    println!("resultado de * é = {:?}", res_multi);
}