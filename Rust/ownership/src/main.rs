fn main() {

    alocando_memoria();
}

fn alocando_memoria()  {


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
    
    println!("vec1 {:?} no endereço {:p} do heap", vec1,&vec1);


    vec1.push(0);
    //Mesmo após adicionarmos um valor no vetor o ponteiro da variável continua o mesmo.
    println!("vec1 {:?} no endereço {:p} do heap",vec1 ,&vec1);
    
    
    //Na linha abaixo, será alocado um novo ponteiro na stack apontando para o endereço onde estará alocado os valores de vec2 no heap
    //Após a alocação o valor de vec1 no heap é MOVIDO para o novo endereço de vec2 e o ponteiro de vec1 que estava na stack que apontava para o endereço de vec1 no heap é removido.
    let vec2:Vec<u8> = vec1;
    
    println!("vec2 {:?} no endereço {:p} do heap", vec2,&vec2);
    
    //Isso significa que vec1 a partir daqui não pode mais ser utilizado,o ponteiro de vec1 foi retirada da stack e o valor de vec1 que estava no heap movido para o endereço de vec2
    //Se tentarmos acessar vec1 o compilador irá nos avisar que o valor foi movido
    //vec1.push(1) | Value used after being moved
    //Se realmente precisarmos copiar o valor temos que utilizar
    let vec3:Vec<u8> = vec2.clone();
    //Assim o ponteiro de vec2 que aponta para o endereço onde estão os valores no heap continua e os valores do heap somente copiados para o novo endereço criado para vec3

    println!("vec3 {:?} no endereço {:p} do heap", vec3,&vec3);
}
// x sai do escopo e é retirada da stack.
// y sai do escopo e é retirada da stack.
// vec1 sai do escopo é retirado do heap e o ponteiro removido da stack.
// vec2 sai do escopo é retirado do heap e o ponteiro removido da stack.
// vec3 sai do escopo é retirado do heap e o ponteiro removido da stack.
