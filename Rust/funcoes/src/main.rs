//Em Rust a ordem em que as funções são declaradas não importam
//contanto que elas estejam no mesmo escopo.
//Ou seja você pode declarar funções onde quiser e chama-las onde quiser.

//Note que todas as funções não possuem a palavra chave return
//para retornar algum valor, isso pois rust retorna sempre a ultima declaração.
//Declaração não é igual a expressão.
//Expressões não necessitam de ponto e virgula no final.


//Declarar uma função também é uma declaração.
fn funcao_com_parametro (num: u8) -> u8 {

    //Isso é uma declaração.
    let new_num = num + 1;

    println!("funcao_com_parametro: {}", num);

    //Isso é uma expressão.
    //Em rust não precisamos escrever return para retornar.
    new_num
}


fn main() {

    //Chamar uma função é uma expressão.
    funcao_com_parametro(1);

    funcao_sem_parametro();

}

//Declarar uma função também é uma declaração.
fn funcao_sem_parametro () -> u8 {

    //Isso é uma declaração.
    let num:u8 = 10;


    //Isso é uma declaração.
    println!("funcao_sem_parametro: {}", num);

    //Isso é uma expressão.
    //Em rust não precisamos escrever return para retornar.
    num
}