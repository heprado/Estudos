fn main() {

    let mut string:String = String::from("Oi");
    
    //Como estamos passando uma refêrencia para a string, a ownership dela não é passada para a função.

    //Por isso conseguimos utilizar a variável string aqui, pois ela não foi desalocada após a recebe_string().
    //Note que os endereços são diferentes, isso, pois uma referência é um ponteiro para o ponteiro da variável string na stack.
    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} do heap",stringify!(string), string,&string,string.as_ptr());

    recebe_referencia(&string,stringify!(string));

    //A variavel não foi dropada como podemos ver, isso pois a propriedade da variavel não foi transferida.
    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} do heap apos função recebe_transferencia",stringify!(string), string,&string,string.as_ptr());

    muda_string(&mut string,stringify!(string));

    //A variavel não foi dropada como podemos ver, isso pois a propriedade da variável não foi transferida.
    //O endereço dela no heap continua o mesmo, mas agora o valor da variável no heap foi modificado,
    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} do heap apos função muda_string()",stringify!(string), string,&string,string.as_ptr());

    condicao_de_corrida();

}

//Recebe uma referência de uma string e nome da variável
fn recebe_referencia(string:&String,nome_variavel:&str) {

    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} do heap dentro da função recebe_referencia()",nome_variavel, string,&string,string.as_ptr());

}
//Aqui a referencia é dropada mas o valor continua no heap.


fn muda_string(string:&mut String ,nome_variavel:&str) {

    string.push_str(" a string foi mudada");
    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} do heap dentro da função muda_string()",nome_variavel, string,&string,string.as_ptr());
}
//Aqui a referencia é dropada mas o valor continua no heap.


fn condicao_de_corrida() {

    let mut string:String = String::from("Oi");

    //Podemos ter 2 referencias não mutáveis para uma variável.
    //Isso, pois se a variável não for mutável não teremos problema dela ser modificada do nada.

    //Referencia não mutavel para string2
    let ref1 = &string;

    //Referencia não mutavel para string2
    let ref2 = &string;

    //Se tentarmos por uma referência mutável antes das referências imutáveis serem usadas teremos problemas.
    //Isso, pois não podemos ter uma referência mutável e imutável ao mesmo tempo, para a mesma variável,
    //pois a referência imutável não espera que o valor seja mudado do nada.
    //let ref3 = &mut string;

    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} do heap",stringify!(ref1), ref1,&ref1,ref1.as_ptr());
    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} do heap",stringify!(ref2), ref2,&ref2,ref2.as_ptr());

    //Aqui as variáveis ref1 e ref2 são dropadas,
    //pois, o escopo de uma referência começa quando elas são declaradas e terminam na última vez que foram usadas.

    //Agora podemos ter uma referência mutável para a variável
    let ref3 = &mut string;

    
    ref3.push_str(" dnv");

    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} do heap",stringify!(ref3), ref3,&ref3,ref3.as_ptr());
}