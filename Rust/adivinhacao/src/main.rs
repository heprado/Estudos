use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::{thread, time};

fn main() {

    let numero = rand::thread_rng() // Gera um número aleatorio utilizando o tempo local da thread que está rodando.
    .gen_range(1..=1000); //Número de 1 até 1000
    
    //Titulo
    println!("Adivinhe o número!");

    //Roda infitamente.
    loop {

        //Printa na tela e pula uma linha
        // \x1b[32m é verde.
        // \x1b[0m é RESETAR A COR.
        println!("\nPor favor insira sua escolha e aperte \x1b[32m Enter \x1b[0m");

        //Let cria uma variavel que só existe neste escopo
        //essa variavel é mutavel por conta do mut
        //todas as variaveis em Rust são imutaveis por padrão.
        //Imutabilidade sendo o valor dela poder ser alterado.
        //new() é uma função implementada no Tipo String
        let mut escolha = String::new();

        
        //Começa a ler o STDIN.
        io::stdin()
            .read_line(&mut escolha) //Passando a escolha por referencia (mutavel) para o readline
            .expect("Erro ao ler a linha.");

        //Pega a string do resultado, remove os espaços em branco e transforma a string em um u32, sempre o tipo que você definir na variável que será o tipo que o parse tentará transformar.
        //Parse retorna um Result<Ok,Err>
        let escolha: u32 = match escolha.trim().parse() {
            //Pega o valor retornado caso tenha sido OK. 
            Ok(num) => num, 
            //Fala que deve ser um número para qualquer(isso que o underline faz) erro que acontencer
            Err(_) => { 
                println!("\x1b[33mDigite um número \x1b[0m"); 
                continue 
            }, 
        };
        
        //Passa o numero por referencia para comparar com a escolha, utilizando o metodo cmp do u32;
        match escolha.cmp(&numero) {

            //Menor
            Ordering::Less => println!("\n\x1b[36mNúmero é maior\x1b[0m 😌"),
            //Maior
            Ordering::Greater => println!("\x1b[31mNúmero é muito grande\x1b[0m 😨"),
            //Igual
            Ordering::Equal =>  { 
                println!("\n\x1b[35mVocê acertou!!!\x1b[0m 🤑🤑🤑");

                
                // Aguarda 10segundos antes de sair
                thread::sleep(time::Duration::from_millis(10000));
                // Sai do programa caso acerte.
                break; 
                
            }
        }
    
    }
    

}