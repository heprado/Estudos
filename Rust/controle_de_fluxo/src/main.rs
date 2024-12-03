
fn if_else_if () {
    let condicao:bool = true;

    if condicao {
        println!("Condição é verdadeira.")
    }
    else {
        println!("Condição é falsa.")
    }

    let num:u8 = 3;

    //Em rust nenhuma variavel é transformada magicamente
    //para um booleano, então é impossivel fazer o seguinte:
    //if num {
    //  println!("Número existe.");
    //}
    if num != 0 {
        println!("Número é outro sem ser 0!");
    }

    if num == 0 {
        println!("Número é zero!");
    }
    //Else if só é executado caso a condição anterior seja falsa.
    else if num > 0 {
        println!("Número é maior que zero!");
    }
    else {
        println!("Número é menor que zero!");
    }
}

fn if_ternario () -> bool {

    //Também podemos utilizar o if dentro da declaração de uma variavel,
    //isso é chamado de if ternario em outras linguagens.
    //Os valores dentro das condições PRECISAM SER DO MESMO TIPO.
    let um_maior_que_zero:bool = if 1 > 0 {true} else {false};

    um_maior_que_zero
}

fn main() {

    if_else_if();
    if_ternario();


}
