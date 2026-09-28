
/*
As 3 regras do Ownership
Todo valor em Rust tem um dono (uma variável).
Só pode haver um dono por vez.
Quando o dono sai do escopo (chega no }), o valor é liberado automaticamente.
 */

fn main(){

{
    let _nome = String::from("My name is daniel");
    println!("Name is {}", _nome);

}// Aqui e dropado o valor que esta na heap do ponteiro nome.
// Está parte equivale a alocar um espaço em mémoria, e retorna o endereço para o ponteiro 
// Free automáticamente após o sim da chave 


//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// Move 
let dadoss = String::from("Olá mundo"); 
// Aqui a um espaço heap com os dados string 
// dadoss nao e dono deste valor, é sim contém o endereco para acessar


let s3 = dadoss;
// Neste caso s3 recebe o endereco da string olá mundo 
// Ele passa ser dono deste endereco 
// dadoss nao contém nada, está liberado !
// Portanto s3 e o dono agora 

 // println!("{}", dadoss); // Erro 
println!("{}", s3);


//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
let your = String::from("Carlos daniel");
let my = your.clone();

println!("{}", your); // Funciona corretamente 
println!("{}", my); // funciona corretamente 

// Clone() Copiou os dados de your, mas estes dados nao estao no memso espaço de memoria 
// Clone() copiou e allocou e atribuiu a o ponteiro my
// Os dados sao os mesmo mas o espaco onde cada um está é diferente 

}