
use iocaptando::input;


fn main() {
    println!("Digite um texto");
    let texto = input();

    println!("Texto e {}", texto);
}