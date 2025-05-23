fn main() {

    literais_vs_mutaveis();
    ownership();
}

fn literais_vs_mutaveis()  {


    //Valores literais como &str, números(u8,i8,u16,i16,etc...) são adicionados a stack.
    let x:u8 = 1;
    let y:u8 = x;
    
    //Note que os endereços são diferentes mesmo a variável y sendo exatamente o valor da variável x
    //Isso, pois y foi alocado um endereço na stack e o valor de x copiado para este novo endereço.
    //Isso somente acontece com valores literais, pois copiar valores já conhecidos na hora da compilação não é custoso.
    println!("x {} no endereço {:p} da stack", x,&x);
    println!("y {} no endereço {:p} da stack", y,&y);
    
    //Quando criamos valores mutáveis, o rust adiciona um ponteiro na stack que aponta para o endereço onde está alocado os valores no heap.
    let mut vec1:Vec<u8> = vec![0;10];
    
    println!("vec1 {:?} no endereço {:p} da stack e no endereço do heap {:p}", vec1,&vec1,vec1.as_ptr());


    vec1.push(0);
    //Mesmo após adicionarmos um valor no vetor o ponteiro da variável na stack continua o mesmo.
    //O do heap muda, pois valores foram adicionados e precisa refletir essas mudanças.
    println!("vec1 {:?} no endereço {:p} da stack apontando para o endereco {:p} do heap ",vec1 ,&vec1,vec1.as_ptr());
    
    
    //Na linha abaixo, será alocado um novo ponteiro na stack apontando para o endereço onde estará alocado os valores de vec2 no heap
    //Após a alocação o valor de vec1 no heap é MOVIDO para o novo endereço de vec2 e o ponteiro de vec1 que estava na stack que apontava para o endereço de vec1 no heap é removido.
    let vec2:Vec<u8> = vec1;
    //Note que o endereço do heap é o mesmo da vec1.
    println!("vec2 {:?} no endereço {:p} da stack apontando para o endereço {:p} do heap", vec2,&vec2,vec2.as_ptr());
    
    //Isso significa que vec1 a partir daqui não pode mais ser utilizado,o ponteiro de vec1 foi retirada da stack e o valor de vec1 que estava no heap movido para o endereço de vec2
    //Se tentarmos acessar vec1 o compilador irá nos avisar que o valor foi movido
    //vec1.push(1) | Value used after being moved
    //Se realmente precisarmos copiar o valor temos que utilizar
    let vec3:Vec<u8> = vec2.clone();
    //Assim o ponteiro de vec2 que aponta para o endereço onde estão os valores no heap continua e os valores do heap somente copiados para o novo endereço criado para vec3
    println!("vec2 {:?} no endereço {:p} da stack apontando para o endereço {:p} do heap", vec2,&vec2,vec2.as_ptr());
    println!("vec3 {:?} no endereço {:p} da stack apontando para o endereço {:p} do heap", vec3,&vec3,vec3.as_ptr());
}
// x sai do escopo e é retirada da stack.
// y sai do escopo e é retirada da stack.
// vec1 sai do escopo é retirado do heap e o ponteiro removido da stack.
// vec2 sai do escopo é retirado do heap e o ponteiro removido da stack.
// vec3 sai do escopo é retirado do heap e o ponteiro removido da stack.

fn ownership() {
    
    
    let string1 = String::from("hello");

    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} no heap antes da função pega_ownership()",stringify!(string1),string1,&string1,string1.as_ptr());
   
    //Quando passamos uma variável de um tipo mutável numa função
    //A função pega propriedade da variável.
    //O endereço dentro da pega_ownership é diferente, pois, foi criado uma nova refêrencia na stack apontando para o endereço da string1 no heap.
    //Note que o endereço do heap é o mesmo.
    pega_ownership(string1,stringify!(string1));
    //Ou seja, não podemos mais utilizar a variável string1 nesse escopo.

    let numero1:u8 = 2;
    
    copia_variavel(numero1,stringify!(numero1));
    
    //Note que o endereço é diferente.
    //Isso, pois ela foi copiada.
    //Isso só é possível em tipos que implementam a trait Copy.
    println!("{} {} no endereço {:p} da stack após copia_variavel()",stringify!(numero1),numero1,&numero1);
    
    let string2 = entrega_ownership();
    
    //O endereço é diferente, isso ocorre, pois é adicionado um novo ponteiro na stack apontando 
    //para o endereço do heap onde o valor da variável string2 está.
    //Note que o endereço do heap é o mesmo
    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} do heap após entrega_variavel()",stringify!(string2),string2,&string2,string2.as_ptr());
    
    let string3 = String::from("hello2");
    
    //O endereço é diferente, isso ocorre, pois é adicionado um novo ponteiro na stack apontando 
    //para o endereço do heap onde o valor da variável string3 está.
    let string4 = pega_ownership_e_devolve(string3,stringify!(string3));
    
    //Note que o endereço do heap é o mesmo, mesmo sendo outra variável.
    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} do heap após pega_ownership_e_devolve()",stringify!(string4),string4,&string4,string4.as_ptr());
    
}

fn pega_ownership(texto:String,nome_variavel:&str) {
    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} no heap na função função pega_ownership()", nome_variavel,texto,&texto,texto.as_ptr());
}

fn copia_variavel(numero:u8,nome_variavel:&str) {
    println!("{} {} no endereço {:p} dentro da função copia_variavel()",nome_variavel,numero,&numero);
}

fn entrega_ownership() -> String {
    
    //Retorna a String hello.
    let string = String::from("hello");
    
    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} do heap dentro da entrega_variavel()",stringify!(string),string,&string,string.as_ptr());
    
    string
    //string sai do escopo seu ponteiro e espaço no heap é descartado.
}


fn pega_ownership_e_devolve(texto:String,nome_variavel:&str) -> String {
    println!("{} {} no endereço {:p} da stack apontando para o endereço {:p} do heap dentro da função pega_variavel_e_devolve()",nome_variavel,texto,&texto,texto.as_ptr());
    texto
}