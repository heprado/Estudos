use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::{thread, time};

fn main() {

    let numero = rand::thread_rng() // Gera um número aleatório utilizando o tempo local da thread que está rodando.
    .gen_range(1..=u128::MAX);

    let mut tentativas = 10;
    //Titulo


    println!("Adivinhe o número entre 1 e {} ",u128::MAX);

    //Roda infinitamente.
    loop {

        if tentativas < 1 {

            print!("\x1Bc");
            println!("\x1b[31mAcabou pra você.\x1b[0m");
            println!("O número era \x1b[32m{}\x1b[0m",numero);
            thread::sleep(time::Duration::from_millis(2000));
            break;
        }

        //Printa na tela e pula uma linha
        // \x1b[32m é verde.
        // \x1b[0m é RESETAR A COR.
        println!("\nPor favor insira sua escolha e aperte \x1b[32mEnter\x1b[0m");

        //Let cria uma variável que só existe neste escopo
        //essa variável é mutável por conta do mut
        //todas as variáveis em Rust são imutáveis por padrão.
        //Imutabilidade sendo o valor dela poder ser alterado.
        //new() é uma função implementada no Tipo String
        let mut escolha = String::new();

        
        //Começa a ler o STDIN.
        io::stdin()
            .read_line(&mut escolha) //Passando a escolha por refêrencia (mutável) para o readline
            .expect("Erro ao ler a linha.");

        //Pega a string do resultado, remove os espaços em branco e transforma a string em um u32, sempre o tipo que você definir na variável que será o tipo que o parse tentará transformar.
        //Parse retorna um Result<Ok, Err>
        let escolha: u128 = match escolha.trim().parse() {
            //Pega o valor retornado caso tenha sido OK. 
            Ok(num) => num, 
            //Fala que deve ser um número para qualquer(isso que o underline faz) erro que acontecer
            Err(_) => {
                print!("\x1Bc");

                println!("\x1b[31mDigite um número até {}\x1b[0m",u128::MAX);
                println!();
                println!("\x1b[33mVocê tem {} tentativas\x1b[0m",tentativas);
                continue 
            }, 
        };



        //Passa o número por refêrencia para comparar com a escolha, utilizando o método cmp do u32;
        match escolha.cmp(&numero) {

            //Menor
            Ordering::Less => {
                tentativas -= 1;
                print!("\x1Bc");
                println!("\x1b[33mVocê tem {} tentativas\x1b[0m",tentativas);
                println!("\n\x1b[36mNúmero é maior\x1b[0m 😌");

            },
            //Maior
            Ordering::Greater => {
                tentativas -= 1;
                print!("\x1Bc");
                println!("\x1b[33mVocê tem {} tentativas\x1b[0m",tentativas);
                println!("\x1b[31mNúmero é muito grande\x1b[0m 😨");
            },
            //Igual
            Ordering::Equal =>  {
                print!("\x1Bc");
                println!("\n\x1b[35mVocê acertou!!!\x1b[0m 🤑🤑🤑");

                
                // Aguarda 10segundos antes de sair
                thread::sleep(time::Duration::from_millis(2000));
                // Sai do programa caso acerte.
                break; 
                
            }
        }
    
    }
    

}