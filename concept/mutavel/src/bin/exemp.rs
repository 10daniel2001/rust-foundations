fn testando(a: i32) -> i32 {
    let numero = a;
    // Numero nao e mútavel, entâo nao posso receber outro valor 
    

    let ress = numero * 10;

    ress
}// Após o escopo e tudo liberado 
//Declarei uma funçâo que recebe um paranmétro inteiro
// 


fn main() {
    let varavel = testando(20);

    println!("{}", varavel);
    // 200 

}