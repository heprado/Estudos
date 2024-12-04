use std::io;


const BASE_FAHRENHEIT:f32 = 32.0;
const BASE_CELSIUS:f32 = 0.0;
const BASE_KELVIN:f32 = 273.15;



fn celsius_para_fahrenheit() {


    loop {

        println!("Digite a temperatura em Celsius:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Erro ao ler a linha.");

        let celsius:f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Erro ao ler celsius.");
                continue
            }
        };

        println!("\n🌡️ Temperatura {}º em Celsius é {}º em Fahrenheit\n",celsius,(1.8 * celsius) + BASE_FAHRENHEIT);


        break
    }

}

fn celsius_para_kelvin() {


    loop {

        println!("Digite a temperatura em Celsius:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Erro ao ler a linha.");

        let celsius:f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Erro ao ler celsius.");
                continue
            }
        };

        println!("\n🌡️ Temperatura {}º em Celsius é {}º em Fahrenheit\n",celsius, celsius + BASE_KELVIN);

        break
    }

}

fn fahrenheit_para_celsius() {

    loop {

        println!("Digite a temperatura em Fahrenheit:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Erro ao ler a linha.");

        let fahrenheit:f32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Erro ao ler a fahrenheit");
                continue;
            }
        };

        println!("\n🌡️ Temperatura {}º em Fahrenheit é {}º em Celsius\n",fahrenheit,(fahrenheit - BASE_FAHRENHEIT) / 1.8);

        break
    }


}

fn fahrenheit_para_kelvin() {


    loop {

        println!("Digite a temperatura em Fahrenheit:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Erro ao ler a linha.");

        let fahrenheit:f32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Erro ao ler a fahrenheit");
                continue;
            }
        };

        println!("\n🌡️ Temperatura {}º em Fahrenheit é {}º em Kelvin\n",fahrenheit,(fahrenheit - BASE_FAHRENHEIT) * 0.5555555555555556 + BASE_KELVIN );

        break
    }

}


fn kelvin_para_celsius() {


    loop {

        println!("Digite a temperatura em Kelvin:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Erro ao ler a linha.");

        let kelvin:f32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Erro ao ler kelvin");
                continue;
            }
        };

        println!("\n🌡️ Temperatura {}º em Kelvin é {}º em Celsius\n",kelvin, kelvin - BASE_KELVIN );

        break
    }

}

fn kelvin_para_fahrenheit() {
    loop {
        println!("Digite a temperatura em Kelvin:");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Erro ao ler a linha.");

        let kelvin: f32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Erro ao ler kelvin");
                continue;
            }
        };

        println!("\n🌡️ Temperatura {}º em Kelvin é {}º em Fahrenheit\n", kelvin, (kelvin - BASE_KELVIN) * 1.8 + BASE_FAHRENHEIT);

        break
    }
}

fn main() {


    loop {

        let mut option:String = String::new();

        println!("1 - Celsius para Fahrenheit\n\
        2 - Celsius para Kelvin\n\
        3 - Fahrenheit para Celsius\n\
        4 - Fahrenheit para Kelvin\n\
        5 - Kelvin para Celsius\n\
        6 - Kelvin para Fahrenheit\n\
        7 - Sair\n\
        \x1b[35mEscolha a opção:\x1b[0m");

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
                celsius_para_fahrenheit();
            }
            2 => {
                celsius_para_kelvin();
            }
            3 => {
                fahrenheit_para_celsius();
            }
            4 => {
                fahrenheit_para_kelvin();
            }
            5 => {
                kelvin_para_celsius();
            }
            6 => {
                kelvin_para_fahrenheit();
            }
            7 => {
                break;
            }
            _ => {
                println!("\x1b[31mOpção Invalida\x1b[0m");
            }
        }

    }
}
