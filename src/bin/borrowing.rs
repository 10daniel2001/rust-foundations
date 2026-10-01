/*Empréstimo em rust 
  
*/


fn main() {
    
    let name = String::from("Carlos daniel");
    let calcular_sze = borrowinf(&name);

    println!("Tamanho da string e -> {}", calcular_sze);
}

fn borrowinf(nam: &String) -> usize {
    nam.len()
}