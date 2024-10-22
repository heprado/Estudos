fn main() {

    //Caso formos tipar uma variavel que contem uma tupla
    //Precisamos especificar o tipo de cada valor.
    //Os valores podem ser alterados (caso a variavel sejá mutavel),
    //só necessitam ser do mesmo tipo.
    let tup:(u8,u8,u8) = (1,2,3);

    //Descontruindo os valores de dentro da variavel tup em 3 variaveis.
    //Underline somente para o compilar ignorar que não estamos usando essas variaveis.
    let (_x,_y,_z) = tup;

    //Caso formos tipar uma variavel que contem um array
    //Precisamos especificar o tipo dos valores do array e o tamanho dele
    //Arrays possuem tamanhos fixos.
    //Arrays só podem ser de um tipo.
    let arr: [u8;10] = [1,2,3,4,5,6,7,8,9,10];

    //Todos os valores de um Vetor devem possuir o mesmo tipo.
    //A principal diferença entre um vetor e um array é que o
    //array não aumenta conforme colocamos mais valores.
    let mut vec: Vec<u8> = vec![1,2,3,4,5,6,7,8,9,10];

    //Colocando mais dois valores no vetor pois é possivel.
    vec.push(11);
    vec.push(12);

    println!("Tupla: {:#?}\n\
    Array: {:#?}\n\
    Vetor: {:#?}",tup,arr,vec);
}
