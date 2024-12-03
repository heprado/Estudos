
fn for_loop() {

    let arr:[u8;3] = [1,2,3];

    //Loop até o final de um array
    for i in arr {
        println!("{}", i);
    }

    //Loop até o final de um Range definido na declaração do loop.
    for i in 1..4 {
        println!("{}", i);
    }
}

fn while_condicional () {

    let mut num = 0;

    //Igual a maioria das linguagens de programação, o while
    //ira rodar até a condição ser falsa.
    while num <= 10 {
        
        println!("Number : {}",num);
        num += 1;
    }
}

fn loop_labels () {

    //Contador de iterações do loop externo.
    let mut outer_count = 0;

    //Loop principal,
    //Podemos nomear um loop com labels utilizando o apostrofo '.
    'outer_loop: loop {

        println!("Contador loop Externo = {outer_count}");

        //Contador do loop interno
        let mut inner_counter = 10;

        loop {

            println!("Contador loop interno = {inner_counter}");

            if inner_counter == 9 {
                break; //Sai do loop interno e continua o loop externo.
            }

            if outer_count == 5 {
                break 'outer_loop; //Sai do loop externo
            }

            //Subtraindo do contador do loop interno
            inner_counter -= 1;
        }

        outer_count += 1;
    }

    println!("Contador final = {outer_count}");

}
fn loop_return_valor () -> String {

    let palavra:String;

    let mut num = 0;

    loop {
        //Somando um para conseguirmos sair do loop.
        num += 1;

        println!("num: {}", num);

        //Nessa caso se o numéro for igual a 10 vamos sair do loop.
        if num == 10 {
            palavra = String::from("Return");
            //Logo após sairmos do loop atribuimos um valor a variavel
            //Esse return termina a função atual e retorna a string.
            return palavra;
        }
    }

}

fn loop_break_atribuindo_valor () -> String {

    let palavra:String;

    let mut num = 0;

    loop {
        //Somando um para conseguirmos sair do loop.
        num += 1;

        println!("num: {}", num);

        //Nessa caso se o numéro for igual a 10 vamos sair do loop.
        if num == 10 {
            //Logo após sairmos do loop atribuimos um valor a variavel
            break palavra = String::from("Break");
        }
    }

    //Aqui retornamos a variavel.
    palavra
}

fn loop_simples () {

    let mut num = 0;

    // Esse loop roda infinitamente até termos um break.
    loop {

        //Somando um para conseguirmos sair do loop.
        num += 1;

        println!("num: {}", num);

        //Nessa caso se o numéro for igual a 10 vamos sair do loop.
        if num == 10 {
            break;
        }
    }
}

fn main() {
    loop_simples();
    let break1 = loop_break_atribuindo_valor();
    let return1 = loop_return_valor();

    println!("break1: {}", break1);
    println!("return1: {}", return1);

    loop_labels();
}
