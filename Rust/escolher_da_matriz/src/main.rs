
use std::io;

fn main() {

    const ARRAY_SIZE: usize = 10;

    let array : [u8; ARRAY_SIZE] = [192,168,10,1,205,10,10,10,10,1];

    loop {

        let mut pick = String::new();

        println!("Escreva o index em que você deseja ver o valor:");

        io::stdin()
            .read_line(&mut pick)
            .expect("Erro ao ler a linha.");

        let index: usize = match pick.trim()
            .parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\x1b[33mDigite um número \x1b[0m");
                continue;
            }
        };

        if index >= ARRAY_SIZE {
            println!("Index deve ser menor que {}",ARRAY_SIZE);
            continue;
        }

        let picked : u8 = array[index];

        println!("Valor do index {} é {}",index,picked);
    }

}
