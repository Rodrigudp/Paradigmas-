use std::io::{self, Write};

fn main() {
    println!("Ola, mundo!");

    print!("Digite um numero para ver a tabuada: ");
    let _ = io::stdout().flush();

    let mut entrada = String::new();
    let _ = io::stdin().read_line(&mut entrada);

    let numero = match entrada.trim().parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Entrada invalida ou vazia. Usando o numero 5 como exemplo.");
            5
        }
    };

    println!("\nTabuada do {}:", numero);
    for i in 1..=10 {
        println!("{} x {} = {}", numero, i, numero * i);
    }
}
