use std::io;


const BASE_FAHRENHEIT:f32 = 32.0;
const BASE_CELSIUS:f32 = 212.0;

fn fahrenheit_para_celsius() {


    loop {

        println!("Digite a temperatura em Fahrenheit:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Erro ao ler a linha.");

        let fahrenheit:f32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Er ao ler a fahrenheit");
                continue;
            }
        };

        println!("Temperatura {} em Fahrenheit é {} em Celsius",fahrenheit,(fahrenheit - BASE_FAHRENHEIT) / 1.8);

        break
    }


}


fn celsius_para_fahrenheit() {


    loop {

        println!("Digite a temperatura em Celsius:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Erro ao ler a linha.");

        let celsius:f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Erro ao ler a linha.");
                continue
            }
        };

        println!("Temperatura {} em Celsius é {} em Fahrenheit",celsius,(1.8 * celsius) + BASE_FAHRENHEIT);

        break
    }

}

fn main() {


    loop {

        let mut option:String = String::new();

        println!("1 - Fahrenheit para Celsius\n2 - Celsius para Fahrenheit\n3 - Sair\n\x1b[35mEscolha a opção:\x1b[0m");

        io::stdin().read_line(&mut option).expect("Erro ao ler a linha.");

        let choice:u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\x1b[33mDigite um número \x1b[0m");
                continue
            },
        };

        match choice {
            1 => {
               fahrenheit_para_celsius();
            }
            2 => {
                celsius_para_fahrenheit();
            }
            3 => {
                break;
            }
            _ => {
                println!("\x1b[31mOpção Invalida\x1b[0m");
            }
        }

    }
}
